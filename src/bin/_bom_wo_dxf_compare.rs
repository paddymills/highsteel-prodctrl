
use bb8::Pool;
use bb8_tiberius::ConnectionManager;

// use rayon::prelude::*;
use tokio::sync::mpsc;

use std::{collections::BTreeMap};
use linya::Progress;
use simple_excel_writer::*;
use uncased::Uncased;

use prodctrl::{db, part::Part};

#[tokio::main]
async fn main() -> Result<(), prodctrl::Error> {
    // console_subscriber::init();

    let bom_pool = Pool::builder()
        .build(ConnectionManager::build(db::HssConfig::Bom)?)
        .await?;

    let sndb_pool = Pool::builder()
        .build(ConnectionManager::build(db::HssConfig::Sigmanest)?)
        .await?;

    // get a list of jobs/shipments currently in workorders
    let jobs: Vec<JobShip> = sndb_pool.get().await?
        .simple_query("
            SELECT DISTINCT Data1, Data2
            FROM Part
            WHERE WONumber LIKE '%-%' AND Data1 LIKE '1%'

            AND WONumber LIKE '12%'
        ").await?
        .into_first_result().await?
        .into_iter()
        .map( |row| JobShip::from(&row) )
        .collect();

    // progress bar
    let mut progress = Progress::new();
    let jobs_bar = progress.bar(jobs.len(), "Jobs");
    progress.draw(&jobs_bar);

    // get parts per job/shipment
    let (tx, mut rx) = mpsc::channel(32);
    let mut bom_conn = bom_pool.get().await?;
    let mut sndb_conn = sndb_pool.get().await?;
    let mut tasks = 0usize;
    for js in jobs {
        
        // get work order quantities
        let res = sndb_conn
            .query(
                "
                    SELECT
                        PartName AS Piecemark,
                        QtyOrdered AS Qty
                    FROM Part
                    WHERE Data1=@P1 AND Data2=@P2
                    AND WONumber LIKE '%-%'
                ", &[&js.job, &js.ship]
            ).await?
            .into_first_result().await?
            .into_iter()
            .map(|row| (Process::WorkOrder(get_qty(&row)), get_mark(&row)) );

        for (proc, mark) in res {
            let tx = tx.clone();
            let js = js.clone();

            tokio::spawn(async move { tx.send( (proc, js, mark) ).await });

            tasks += 1;
        }

        // get bom quantities
        let res = bom_conn
            .query(
                "EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2",
                &[&js.job, &js.ship]
            ).await?
            .into_first_result().await?
            .into_iter()
            .map(|row| Part::from(row))
            .filter(|part| part.is_pl())
            .map(|part| (Process::Bom(part.qty), part.mark) );

        for (proc, mark) in res {
            let tx = tx.clone();
            let js = js.clone();

            tokio::spawn(async move { tx.send( (proc, js, mark) ).await });

            tasks += 1;
        }

        progress.inc_and_draw(&jobs_bar, 1);
    }

    // close communication channel
    drop(tx);
    
    let parts_bar = progress.bar(tasks, "Parts");
    progress.draw(&parts_bar);

    let mut map = BTreeMap::<JobShip, BTreeMap<Uncased, PartCompare>>::new();
    while let Some((process, js, mark)) = rx.recv().await {
        // bar.println(format!("Processing {:?} {}, {}", process, js, mark));
        let mark = Uncased::from_owned(mark);

        if !map.contains_key(&js) {
            map.insert(js.clone(), BTreeMap::<Uncased, PartCompare>::new());
        }

        if !map[&js].contains_key(&mark) {
            map.get_mut(&js)
                .expect("failed to get job-ship map")
                .insert(mark.clone(), PartCompare { ..Default::default() });
        }

        let part_map = map
            .get_mut(&js).expect("failed to get job-ship map")
            .get_mut(&mark).expect("failed to get mark compare");

        match process {
            Process::WorkOrder(qty) => part_map.workorder = qty,
            Process::Bom(qty)       => part_map.bom = qty,
            Process::Dxf            => part_map.dxf = true
        }

        progress.inc_and_draw(&parts_bar, 1);
    }

    // find dxf files

    // show results
    let mut wb = Workbook::create("C:\\temp\\WorkOrder_Bom_Dxf_Compare.xlsx");
    let mut sheet = wb.create_sheet("Compare");
    wb.write_sheet(&mut sheet, |sw| {
        sw.append_row(row!["Job", "Mark", "Work Order", "Bom","Delta", "Dxf"])?;
        for (js, parts) in map.iter() {
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
            // println!("{}-{} ({})", js.job, js.ship, delta);
        }

        Ok(())
    }).expect("Failed to write data");
    wb.close().expect("Failed to close workbook");

    Ok(())
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct JobShip {
    job: String,
    ship: String
}

#[derive(Debug, Default)]
struct PartCompare {
    workorder: i32,
    bom: i32,
    dxf: bool
}

impl From<&tiberius::Row> for JobShip {
    fn from(row: &tiberius::Row) -> Self {
        Self {
            job: row.get::<&str, _>("Data1").expect("Job is None").into(),
            ship: row.get::<&str, _>("Data2").expect("Shipment is None").into()
        }
    }
}

fn get_qty(row: &tiberius::Row) -> i32 {
    row.get::<i32, _>("Qty").expect("Qty is None")
}

fn get_mark(row: &tiberius::Row) -> String {
    let mut mark = row.get::<&str, _>("Piecemark").expect("Mark is None").to_string();
    mark.make_ascii_uppercase();

    if let Some( (_prefix, suffix) ) = mark.split_once("_") {
        return suffix.into();
    }

    mark.into()
}

#[derive(Debug)]
enum Process {
    WorkOrder(i32),
    Bom(i32),
    Dxf
}
