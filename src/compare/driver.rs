
use simple_excel_writer::*;
use simplelog::{LevelFilter, Config, WriteLogger};
use std::fs::File;

use tokio::sync::mpsc;
use rayon::prelude::*;
use std::sync::Mutex;


use super::actors::*;
use super::{ api::{JobShip, PartCompare}, ProgressBars };
use crate::Error;

use std::collections::BTreeMap;
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

        let mut bars = ProgressBars::new();
        bars.tick_all();
        
        let (tx, mut rx) = mpsc::channel(256);
        let db = ActorHandle::new(tx).await;

        db.send( ActorTask::GetJobs );
        while let Some(response) = rx.recv().await {
            debug!("Got response: {:?}", response);

            match response {
                ActorResult::Job(js) => {
                    bars.inc_job();
                    self.map.insert(js.clone(), PartMap::new());

                    db.send( ActorTask::GetJob { js } );
                },
                ActorResult::JobProcessed(_js) => { bars.jobs.inc(); },
                ActorResult::Bom(js, mark, qty) => {

                    self.map
                        .get_mut(&js).unwrap()
                        .insert(mark.clone(), PartCompare { workorder: 0, bom: qty, dxf: false });

                    bars.inc_part();
                    bars.bom.inc();

                    let js = js.clone();
                    let mark = mark.clone();
                    db.send( ActorTask::GetPart { js, mark } );
                },
                ActorResult::WorkOrder(js, mark, qty) => {
                    self.map
                        .get_mut(&js).unwrap()
                        .get_mut(&mark).unwrap()
                        .workorder = qty;

                    bars.sndb.inc();
                },
                ActorResult::Dxf(js, mark) => {
                    self.map
                        .get_mut(&js).unwrap()
                        .get_mut(&mark).unwrap()
                        .dxf = true;

                    bars.dxf_sn.inc();
                },
                ActorResult::NoDxf => {
                    bars.dxf_sn.inc();
                    bars.dxf_fs.total += 1;
                },
                _ => {
                    debug!("Received unmatched response: {:?}", response)
                }
            }

            if db.is_done().await {
                break;
            }
        }

        let dxf_b = Mutex::new(bars.dxf_fs);
        self.map
            .par_iter_mut()
            .for_each(|(js, v)| {
                v.par_iter_mut()
                    .filter(|(_, v)| !v.dxf)
                    .for_each(|(mark, v)| {
                        v.dxf = find_dxf_file(js, mark);
                        dxf_b.lock().unwrap().inc();
                    });
            });

        bars.jobs.finish();
        bars.bom.finish();
        bars.sndb.finish();
        bars.dxf_sn.finish();
        dxf_b.lock().unwrap().finish();

        Ok(self)
    }

    pub fn export(&self) -> Result<&Self, Error> {
        let path = "C:\\temp\\WorkOrder_Bom_Dxf_Compare.xlsx";
        let mut wb = Workbook::create(path);
        let mut sheet = wb.create_sheet("Compare");
        wb.write_sheet(&mut sheet, |sw| {
            sw.append_row(row!["Job", "Mark", "Work Order", "Bom", "Delta", "Dxf"])?;
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
                        v.workorder as f64 - v.bom as f64,
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