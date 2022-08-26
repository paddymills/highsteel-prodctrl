
use glob::glob;

use super::*;
use log::debug;

pub fn find_dxf_file(js: &JobShipment, mark: &String) -> bool {
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