
use async_once::AsyncOnce;
use bb8_tiberius::IntoConfig;
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::*;
use tokio::sync::{mpsc, Mutex};

use linya::Progress;
use simple_excel_writer::*;

use std::collections::BTreeMap;

use super::*;
use crate::{db, Error, part::Part};

lazy_static!(
    static ref BOM: AsyncOnce<Mutex<Client<Compat<TcpStream>>>> = AsyncOnce::new(
        async {
            let bom_cfg = db::HssConfig::Bom.into_config().unwrap();
            let bom_tcp = TcpStream::connect(bom_cfg.get_addr()).await.unwrap();
            bom_tcp.set_nodelay(true).unwrap();
    
            let client = Client::connect( bom_cfg, bom_tcp.compat_write() ).await.unwrap();

            Mutex::new(client)
        }
    );

    static ref SNDB: AsyncOnce<Mutex<Client<Compat<TcpStream>>>> = AsyncOnce::new(
        async {
            let sndb_cfg = db::HssConfig::Sigmanest.into_config().unwrap();

            let sndb_tcp = TcpStream::connect(sndb_cfg.get_addr()).await.unwrap();
            sndb_tcp.set_nodelay(true).unwrap();

            let client = Client::connect( sndb_cfg, sndb_tcp.compat_write() ).await.unwrap();

            Mutex::new(client)
        }
    );
);

type PartMap = BTreeMap<String, PartCompare>;
type JobShipMap = BTreeMap<JobShip, PartMap>;

pub struct BomWoDxfCompare {
    map: JobShipMap
}

#[derive(Debug)]
enum TaskResult {
    Job(JobShip),
    JobSearchComplete,
    Part(JobShip, String, PartCompare),
    PartSearchComplete,
    WorkOrder(JobShip, String, i32),
    #[allow(dead_code)]
    Bom(JobShip, String, i32),
    Dxf(JobShip, String)
}

impl BomWoDxfCompare {
    pub async fn new() -> Self {
        Self { map: JobShipMap::new() }
    }

    pub async fn main(&mut self) -> Result<&mut Self, Error> {
        let (tx, mut rx) = mpsc::channel(32);
    
        let mut outstanding_tasks = 0usize;
        let num_jobs = Self::get_jobs(tx.clone()).await?;
        outstanding_tasks += num_jobs;
        // drop(tx);
        
        let mut progress = Progress::with_capacity(2);
        let bars = vec![
            progress.bar(num_jobs, "Jobs"),
            progress.bar(6000, "Parts"),
        ];

        // let mut num_parts = 0usize;
        while let Some(result) = rx.recv().await {
            use TaskResult::*;

            // println!("{:?}", result);
            let tx = tx.clone();

            // Job -> get parts
            // Part -> get workorder qty
            // WorkOrder -> find dxf -> terminate
            match result {
                Job(js) => {
                    self.map.insert(js.clone(), PartMap::new());
                    // jobs.inc_length(1);

                    tokio::spawn(async move { Self::get_parts(&js, tx).await.unwrap() });
                },
                Part(js, mark, compare) => {
                    self.map.get_mut(&js).unwrap().insert(mark.clone(), compare);
                    // parts.inc_length(1);

                    // num_parts += 1;
                    tokio::spawn(async move { Self::get_sn_qty(&js, mark, tx).await.unwrap() });
                    outstanding_tasks += 1;
                },
                WorkOrder(js, mark, qty) => {
                    self.map.get_mut(&js).unwrap().get_mut(&mark).unwrap().workorder = qty;

                    tokio::spawn(async move { Self::get_dxf(js, mark, tx).await.unwrap() });
                },
                Bom(js, mark, qty) => {
                    self.map.get_mut(&js).unwrap().get_mut(&mark).unwrap().bom = qty;
                },
                Dxf(js, mark) => {
                    self.map.get_mut(&js).unwrap().get_mut(&mark).unwrap().dxf = true;
                    outstanding_tasks -= 1;
                },

                JobSearchComplete  => { progress.inc_and_draw(&bars[0], 1); outstanding_tasks -= 1; },
                PartSearchComplete => { progress.inc_and_draw(&bars[1], 1); outstanding_tasks -= 1; },
            }

            if outstanding_tasks == 0 {
                break;
            }
        }

        Ok(self)
    }

    async fn get_jobs(tx: mpsc::Sender<TaskResult>) -> Result<usize, Error> {
        let count = SNDB
            .get().await
            .lock().await
            .simple_query("
                SELECT DISTINCT Data1, Data2
                FROM Part
                WHERE WONumber LIKE '%-%' AND Data1 LIKE '1%'

                AND WONumber LIKE '12%'
            ").await?
            .into_first_result().await?
            .into_iter()
            .map( |row| JobShip::from(&row) )
            .map( |js| {
                let tx = tx.clone();

                tokio::spawn(async move { tx.send(TaskResult::Job(js)).await });

                1
            })
            .sum();

        Ok(count)
    }

    async fn get_parts(js: &JobShip, tx: mpsc::Sender<TaskResult>) -> Result<(), Error> {
        // get bom quantities
        BOM
            .get().await
            .lock().await
            .query(
                "EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2",
                &[&js.job, &js.ship]
            )
            .await?
            .into_first_result().await?
            .into_iter()
            .map(|row| Part::from(row))
            .filter(|part| part.is_pl())
            .map(|part| TaskResult::Part(js.clone(), part.mark, PartCompare { bom: part.qty, ..Default::default() }) )
            .for_each(|result| {
                let tx = tx.clone();

                tokio::spawn(async move { tx.send( result ).await });
            });

        // for result in res {
        //     let tx = tx.clone();

        //     tokio::spawn(async move { tx.send( result ).await });
        // }

        tokio::spawn(async move { tx.send(TaskResult::JobSearchComplete).await });

        Ok(())
    }

    async fn get_sn_qty(js: &JobShip, mark: String, tx: mpsc::Sender<TaskResult>) -> Result<(), Error> {
        // get work order quantities
        let res = SNDB
            .get().await
            .lock().await
            .query(
                "
                    SELECT
                        PartName,
                        QtyOrdered
                    FROM Part
                    WHERE Data1=@P1 AND Data2=@P2 AND PartName='@P1_@P3'
                    AND WONumber LIKE '%-%'
                ", &[&js.job, &js.ship, &mark]
            ).await?
            .into_first_result().await?
            .into_iter()
            .map(|row| TaskResult::WorkOrder(js.clone(), mark.clone(), get_qty(&row)) );

        for result in res {
            let tx = tx.clone();

            tokio::spawn(async move { tx.send( result ).await });
        }

        tokio::spawn(async move { tx.send(TaskResult::PartSearchComplete).await });

        Ok(())
    }

    async fn get_dxf(js: JobShip, mark: String, tx: mpsc::Sender<TaskResult>) -> Result<(), Error> {

        tokio::spawn(async move { tx.send(TaskResult::Dxf(js, mark)).await });

        Ok(())
    }

    pub fn export(&self) -> Result<&Self, Error> {
        let mut wb = Workbook::create("C:\\temp\\WorkOrder_Bom_Dxf_Compare.xlsx");
        let mut sheet = wb.create_sheet("Compare");
        wb.write_sheet(&mut sheet, |sw| {
            sw.append_row(row!["Job", "Mark", "Work Order", "Bom","Delta", "Dxf"])?;
            for (js, parts) in self.map.iter() {
                let js = format!("{}-{}", js.job, js.ship);
                
                for (mark, v) in parts.iter() {
                    if v.workorder == 0 { continue; }
                    // let mark = format!("{}", mark);
    
                    sw.append_row(row![
                        js.as_str(),
                        mark.as_str(),
                        v.workorder as f64,
                        v.bom as f64,
                        (v.workorder - v.bom) as f64,
                        "-"
                    ])?;
                }
            }
    
            Ok(())
        }).expect("Failed to write data");
        wb.close().expect("Failed to close workbook");

        println!("dumped data to C:\\temp\\WorkOrder_Bom_Dxf_Compare.xlsx");
    
        Ok(self)
    }
}