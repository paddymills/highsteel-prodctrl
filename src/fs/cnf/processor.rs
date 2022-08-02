
use csv::{ReaderBuilder, StringRecord, WriterBuilder};

use linya::Progress;
use rayon::prelude::*;
use std::sync::Mutex;

use regex::Regex;
use std::{
    io::Error,
    path::PathBuf,
};

use super::{paths::*, CnfFileRow, Plant, IssueFileRow};

const HEADERS: [&str; 13] = [
    "Mark", "Job", "PartWbs", "PartLoc", "PartQty", "PartUom", "Matl", "MatlWbs" , "MatlQty", "MatlUom", "MatlLoc", "Plant", "Program"
];
const SKIP_LOCS: [Option<&str>; 2] = [None, Some("R&D")];
const DELIM: u8 = b'\t';

// lazy static globals that are non-const
lazy_static! {
    // regexes
    static ref NOT_IN_SAP: Regex = Regex::new(r"^NO[\d\s\w]+SAP$").expect("failed to build regex");
    static ref VALID_WBS:  Regex = Regex::new(r"D-\d{7}-\d{5}").expect("failed to build regex");
}

#[derive(Debug, Default)]
pub struct ProdFileProcessor {
    reader: ReaderBuilder,
    writer: WriterBuilder
}

impl ProdFileProcessor {
    pub fn new() -> Self {
        let mut reader = ReaderBuilder::new();
        reader
            .delimiter(DELIM);

        let mut writer = WriterBuilder::new();
        writer
            .has_headers(false)
            .delimiter(DELIM);

        Self { reader, writer }
    }

    pub fn process_files(&self) -> Result<(), Error> {
        let files = get_ready_files()?;
        let progress = Mutex::new( Progress::new() );
        
        let bar = {
            let mut prog = progress.lock().unwrap();
            let bar = prog.bar(files.len(), "Reading files");
            prog.draw(&bar);

            bar
        };


        files.into_par_iter().for_each(|file| {
            match self.process_file(&file) {
                Ok(_) => (),
                Err(e) => eprintln!("Failed to parse file: {:?}", e)
            }

            progress.lock().unwrap().inc_and_draw(&bar, 1);
        });


        Ok(())
    }

    /// Modifications:
    /// - Plant 3 material to RAW
    /// - Skip items for material not in SAP
    /// - Non-production pieces to scrap
    /// - SAP part name if different from SN
    pub fn process_file(&self, filepath: &PathBuf) -> Result<(), Error> {

        let mut reader = self.reader.from_path(filepath)?;
        reader.set_headers( StringRecord::from(HEADERS.to_vec()) );
        
        let results = reader.deserialize::<CnfFileRow>();

        // find a way to check if file is empty
        {

            let mut prod_writer = self.writer.from_path( filepath.archive_file().as_path() )?;
            let mut issue_writer = self.writer.from_path( filepath.issue_file().as_path() )?;

            for result in results {
                // TODO: log errors
                let mut record = result.expect("Failed to deserialize row");

                // filter out items based on material location
                if SKIP_LOCS[..].contains(&record.matl_loc.as_deref()) {
                    continue;
                }
    
                // consume all HS02 material from RAW
                if record.plant == Plant::Williamsport {
                    record.matl_loc = Some("RAW".into());
                }
    
                if VALID_WBS.is_match(&record.part_wbs) {
                    // write new file with changes
                    prod_writer.serialize(record)?;
                    prod_writer.flush()?;
                } else {
                    // send to issue file;
                    issue_writer.serialize::<IssueFileRow>(record.into())?;
                    issue_writer.flush()?;
                }
            }
        }

        // move file to backup
        std::fs::copy(filepath, filepath.backup_file().as_path()).expect("failed to backup file");
        // std::fs::remove_file(filepath).expect("failed to remove original file");
    
        Ok(())
    }
}
