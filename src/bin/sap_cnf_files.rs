
use std::{
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader, Error},
    path::PathBuf,
};

use prodctrl::cnf::CnfFileRow;

const SAP_DATA_FILES: &str = r"\\hssieng\SNData\SimTrans\SAP Data Files\test";

#[tokio::main]
async fn main() -> Result<(), prodctrl::Error> {
    let processor = ProdFileProcessor::new();
    processor.process_files();
    
    Ok(())
}

#[derive(Debug, Default)]
struct ProdFileProcessor {
    files: Vec<PathBuf>,
}

impl ProdFileProcessor {
    fn new() -> Self {
        Self {
            files: Self::get_ready_files()
        }
    }

    pub fn get_ready_files() -> Vec<PathBuf> {
        let mut files = Vec::new();

        let dir = PathBuf::from(SAP_DATA_FILES);
        let ext = OsStr::new("ready");

        for entry in std::fs::read_dir(dir).unwrap() {
            match entry {
                Ok(ent) => {
                    if ent.path().extension() == Some(ext) {
                        files.push(ent.path().to_path_buf());
                    }
                },
                Err(_) => (),
            };
        }

        files
    }

    pub fn process_files(self) {
        for file in self.files {
            println!("Processing {:?}", file);
            let _ = Self::process_file(&file);
        }
    }

    pub fn process_file(filename: &PathBuf) -> Result<(), Error> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        loop {
            match reader.read_line(&mut line) {
                Ok(0) => break, // 0 bytes read -> EOF
                Ok(_) => {
                    // do stuff
                    let parsed = CnfFileRow::from(&line);
                    println!("{}", parsed.to_string());

                    line.clear();
                },
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                }
            };
        }

        Ok(())
    }

}
