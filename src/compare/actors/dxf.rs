
use glob::glob;
use tokio::sync::broadcast;

use super::task::*;
use crate::Error;

pub async fn dxf_actor(tx: broadcast::Sender<Task>) -> Result<(), Error> {
    let mut rx = tx.subscribe();
    tx.send(Task::Init(1))?;

    debug!("Dxf actor initialized. awaiting tasks...");

    while let Ok(task) = rx.recv().await {
        match task {
            Task::GetDxf(js, mark) => {
                debug!("Finding dxf for {} > {}", js, mark);

                let tx = tx.clone();
                rayon::spawn(move || {
                    let globs = vec![
                        format!(r"\\hssieng\DATA\HS\JOBS\{}\CAM\**\*{}*.dxf", js.job, mark),
                        format!(r"\\hssieng\Jobs\*\{}\**\DXF\{}*{}*.dxf", js.job, js.job, mark),
                    ];
                    
                    let mut found = false;
    
                    'outer: for path in globs {
                        for entry in glob(&path).expect("failed to build glob") {
                            if let Ok(_) = entry {
                                found = true;
                                tx.send(Task::Dxf(js, mark)).unwrap();
    
                                break 'outer;
                            }
                        }
                    }
    
                    if !found {
                        tx.send(Task::NoDxf).unwrap();
                    }
                });
            },
            Task::Stop => break,
            _ => ()
        }
    }

    Ok(())
}

pub fn find_dxf(js: &JobShip, mark: &String) -> bool {
    debug!("Finding dxf for {} > {}", js, mark);

    let globs: [String; 2] = [
        format!(r"\\hssieng\DATA\HS\JOBS\{job}\CAM\**\*{mark}*.dxf", job=js.job, mark=mark),
        format!(r"\\hssieng\Jobs\20{year}\{job}\Fab\**\DXF\{job}*{mark}*.dxf", year=&js.job[1..=2], job=js.job, mark=mark),
    ];

    for path in globs {
        for entry in glob(&path).expect("failed to build glob") {
            if let Ok(_) = entry {
                return true;
            }
        }
    }

    false
}

