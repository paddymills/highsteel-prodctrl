
use pbr;
use simple_excel_writer::*;
use simplelog::{LevelFilter, Config, WriteLogger};
use tokio::sync::broadcast;
// use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use rayon::prelude::*;

use std::collections::BTreeMap;
use std::fs::File;
use std::sync::Mutex;

use super::task::{JobShip, PartCompare, Task};
use super::actors::*;
use crate::Error;

type PartMap = BTreeMap<String, PartCompare>;
type JobShipMap = BTreeMap<JobShip, PartMap>;

pub struct BomWoDxfCompare {
    map: JobShipMap
}

impl BomWoDxfCompare {
    pub async fn new() -> Self {
        WriteLogger::init(
            LevelFilter::Debug,
            Config::default(),
            File::create("bom_wo_dxf_compare.log").unwrap()
        ).expect("Failed to init logger");

        Self { map: JobShipMap::new() }
    }

    pub async fn main(&mut self) -> Result<&mut Self, Error> {
        let (tx, mut rx)  = broadcast::channel(1_000_000);

        let mb = pbr::MultiBar::new();
        let mut jobs_b = mb.create_bar(0);
        let mut bom_b = mb.create_bar(0);
        let mut sn_b = mb.create_bar(0);
        let mut dxf_b = mb.create_bar(0);

        jobs_b.message("Jobs ");
        bom_b.message("Parts > Bom ");
        sn_b.message("Parts > Sn ");
        dxf_b.message("Parts > Dxf ");

        std::thread::spawn(move || { mb.listen(); });

        let [tx1, tx2, _tx3] = [0; 3].map(|_| tx.clone());
        let actors = vec![
            tokio::spawn(async move { bom_actor(tx1).await.unwrap() }),
            tokio::spawn(async move { sndb_actor(tx2).await.unwrap() }),
            // tokio::spawn(async move { dxf_actor(tx3).await.unwrap() }),
        ];

        let mut total = 0u8;
        let mut tasks_started = false;
        let mut tasks = 0usize;
        loop {
            let result = rx.recv().await;
            
            if let Err(e) = result {
                println!("driver lagged");
                error!("broadcast error: {:?}", e);

                // tx.send(Task::Stop)?;
                // break;
                continue;
            }
            
            debug!("{:?}", result);
            match result.unwrap() {
                Task::Init(i) => {
                    total += i;
                    
                    if total == actors.len() as u8 {
                        debug!("All actors subscribed");
                        tx.send(Task::GetJobs)?;
                    }
                },
                Task::Job(js) => {
                    self.map.insert(js.clone(), PartMap::new());
                    tx.send(Task::GetParts(js))?;
                    
                    jobs_b.total += 1;
                    jobs_b.inc();
                },
                Task::Part(js, mark, cmp) => {
                    self.map.get_mut(&js).unwrap().insert(mark.clone(), cmp);

                    tx.send(Task::GetWorkOrder(js.clone(), mark.clone()))?;
                    tx.send(Task::GetDxf(js.clone(), mark.clone()))?;

                    bom_b.total += 1;
                    sn_b.total += 1;
                    dxf_b.total += 1;
                    
                    bom_b.inc();
                    sn_b.tick();
                    dxf_b.tick();

                    tasks_started = true;
                    tasks += 2;
                },
                Task::WorkOrder(js, mark, qty) => {
                    self.map.get_mut(&js).unwrap().get_mut(&mark).unwrap().workorder += qty;

                    sn_b.inc();
                    tasks -= 1;
                },
                Task::Dxf(js, mark) => {
                    self.map.get_mut(&js).unwrap().get_mut(&mark).unwrap().dxf = true;

                    dxf_b.inc();
                    tasks -= 1;
                },
                Task::NoDxf => {
                    // dxf_b.inc();
                    tasks -= 1;
                },
                _ => ()
            };

            if tasks_started && tasks == 0 {
                tx.send(Task::Stop)?;
                break;
            }
        }

        let dxf_b = Mutex::new(dxf_b);
        self.map
            .par_iter_mut()
            .for_each(|(js, v)| {
                v.par_iter_mut()
                    .filter(|(_, v)| !v.dxf)
                    .for_each(|(mark, v)| {
                        v.dxf = find_dxf(js, mark);
                        dxf_b.lock().unwrap().inc();
                    });
            });

        Ok(self)
    }

    pub fn export(&self) -> Result<&Self, Error> {
        let path = "C:\\temp\\WorkOrder_Bom_Dxf_Compare.xlsx";
        let mut wb = Workbook::create(path);
        let mut sheet = wb.create_sheet("Compare");
        wb.write_sheet(&mut sheet, |sw| {
            sw.append_row(row!["Job", "Mark", "Work Order", "Bom","Delta", "Dxf"])?;
            for (js, parts) in self.map.iter() {
                let js = format!("{}-{}", js.job, js.ship);
                
                for (mark, v) in parts.iter() {
                    // if v.workorder == 0 { continue; }
                    // let mark = format!("{}", mark);
    
                    sw.append_row(row![
                        js.as_str(),
                        mark.as_str(),
                        v.workorder as f64,
                        v.bom as f64,
                        (v.workorder - v.bom) as f64,
                        if v.dxf { "YES" } else { "NO" }
                    ])?;
                }
            }
    
            Ok(())
        }).expect("Failed to write data");
        wb.close().expect("Failed to close workbook");

        println!("dumped data to {}", path);
    
        Ok(self)
    }
}