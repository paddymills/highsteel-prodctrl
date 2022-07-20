
use simple_excel_writer::*;
use simplelog::{LevelFilter, Config, WriteLogger};
use std::fs::File;

use crossbeam::channel;
use rayon::prelude::*;
use tokio::sync::mpsc;
use std::sync::Mutex;
use std::thread;

use super::actors::*;
use super::{
    api::{find_dxf_file, SnDbOps},
    JobShipMap,
    ProgressBars
};
use crate::Error;

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
        
        let jobs = get_sndb_pool().await
            .get().await
                .expect("Failed to get Sn db client")
            .get_jobs().await;

        let (txq, rxq) = channel::bounded(16);
        for js in jobs {
            bars.inc_job();
            
            let tx = txq.clone();
            thread::spawn(move || {
                let _ = tx.send(js);
            });
        }
        drop(txq);
        
        let (tx, mut rx) = mpsc::channel(16);
        tokio::spawn( spawn_actors(tx, rxq) );

        while let Some(res) = rx.recv().await {
            debug!("Received result: {}", res.js);

            let _ = self.map.insert(res.js, res.parts);
            bars.inc_job();
        }

        let bars = Mutex::new(bars.dxf_fs);
        self.map
            .par_iter_mut()
            .for_each(|(js, v)| {
                v.par_iter_mut()
                    .filter(|(_, v)| !v.dxf)
                    .for_each(|(mark, v)| {
                        v.dxf = find_dxf_file(js, mark);
                        bars.lock().unwrap().inc();
                    });
            });

        bars.lock().unwrap().finish();

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