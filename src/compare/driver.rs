
use simple_excel_writer::*;
use pbr::MultiBar;

// use log::Log;
use simplelog::{LevelFilter, Config, WriteLogger};

use crossbeam::channel;
use rayon::prelude::*;
use tokio::sync::{mpsc, oneshot};
use std::thread;

use tokio::sync::Mutex;
use std::sync::Arc;

use super::{actors::*, PartMap};
use super::{
    api::{find_dxf_file, SnDbOps},
    JobShipMap
};
use crate::Error;

pub struct BomWoDxfCompare {
    map: JobShipMap
}

enum Progress {
    AddJob,
    AddPart,
    IncJob,
    IncPart,
    Finish
}

impl BomWoDxfCompare {
    pub async fn new() -> Self {
        WriteLogger::init(
            LevelFilter::Debug,
            Config::default(),
            std::fs::File::create("bom_wo_dxf_compare.log").expect("failed to create log")
            // std::io::stderr()
        ).expect("Failed to init logger");

        Self { map: JobShipMap::new() }
    }

    pub async fn main(&mut self) -> Result<&mut Self, Error> {

        let prog = {
            let show_bars = true;
            let (tx, mut rx) = mpsc::channel(64);

            if show_bars {
                let mb = MultiBar::new();
                let mut jobs = mb.create_bar(0);
                let mut parts = mb.create_bar(0);
                jobs.message("Jobs ");
                parts.message("Parts ");
                
                thread::spawn(move || { mb.listen(); });
                
                tokio::spawn(async move {
                    while let Some(n) = rx.recv().await {
                        match n {
                            Progress::AddJob  => jobs.total += 1,
                            Progress::AddPart => parts.total += 1,
                            Progress::IncJob  => { jobs.inc(); },
                            Progress::IncPart => { parts.inc(); },
                            Progress::Finish  => {
                                jobs.finish();
                                parts.finish();
                                break;
                            }
                        }
            
                        jobs.tick();
                        parts.tick();
                    }
                });

            } else {
                tokio::spawn(async move {
                    while let Some(_) = rx.recv().await {
                        ()
                    }
                });
            }
            

            tx
        };

        let map = Arc::new(Mutex::new(JobShipMap::new()));
        
        let (tx, mut rx) = mpsc::channel(1024);
        let (tx0, rx0) = channel::bounded(1024);
        
        let jobs = get_sndb_pool().await
            .get().await
                .expect("Failed to get Sn db client")
            .get_jobs().await;

        for js in jobs {
            let _ = prog.send(Progress::AddJob).await;

            let (txj, txp) = (tx0.clone(), tx0.clone());
            let (tx1, rx1) = oneshot::channel();

            thread::spawn(move || {
                if txj.is_full() { debug!("queue full") }

                let msg = format!("Sent {} into queue", js);
                let _ = txj.send(Message::GetJobShip { js, respond_to: tx1 });
                debug!("{}", msg);
            });
            
            let cloned = map.clone();
            let tx = tx.clone();
            let prog = prog.clone();
            tokio::spawn(async move {
                match rx1.await {
                    Ok(res) => {
                        debug!("Got response: {:?} ({})", res.js, res.parts.len());
                        let _ = prog.send(Progress::IncJob).await;

                        let js = res.js;

                        let mut lock = cloned.lock().await;
                        lock.insert(js.clone(), PartMap::new());

                        for part in res.parts {
                            let _ = prog.send(Progress::AddPart).await;

                            let tx0 = txp.clone();
                            let (tx1, rx1) = oneshot::channel();

                            let js = js.clone();
                            thread::spawn(move || {
                                let (mark, compare) = part;
                                if tx0.is_full() { debug!("queue full") }

                                let msg = format!("Sending {}_{} into queue", &js, &mark);
                                let _ = tx0.send(Message::GetPartData { js, mark, compare, respond_to: tx1 });
                                debug!("{}", msg)
                            });

                            let tx = tx.clone();
                            let prog = prog.clone();
                            tokio::spawn(async move {
                                match rx1.await {
                                    Ok(res) => {
                                        let _ = tx.send(res).await;
                                        let _ = prog.send(Progress::IncPart).await;
                                    },
                                    Err(e) => { error!("Got error: {:?}", e) }
                                }
                            });
                        }
                    },
                    Err(e) => { error!("Got error: {:?}", e) }
                }
            });
        }

        // spawn workers
        for _ in 0..32 {
            let _ = BothActor::new(rx0.clone());
        }

        drop(tx0);
        drop(tx);

        while let Some(res) = rx.recv().await {
            let PartResults { js, mark, compare } = res;

            map
                .lock().await
                .get_mut(&js).expect("failed to get job from map")
                .insert(mark, compare);
        }

        
        let _ = prog.send(Progress::Finish).await;

        let lock = map.lock().await;
        self.map = lock.clone();

        println!("checking filesystem for dxf files...");
        self.map
            .par_iter_mut()
            .for_each(|(js, v)| {
                v.par_iter_mut()
                    .filter(|(_, v)| !v.dxf)
                    .for_each(|(mark, v)| {
                        v.dxf = find_dxf_file(js, mark);
                    });
            });

        println!("Processing complete");

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