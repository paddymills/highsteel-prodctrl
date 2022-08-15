
use csv::{ReaderBuilder, StringRecord, WriterBuilder};

use linya::Progress;
use rayon::prelude::*;
use std::sync::Mutex;

use regex::Regex;
use std::{
    io::Error,
    path::PathBuf,
};

use crate::{CnfFileRow, IssueFileRow, Plant};
use super::paths::*;

const HEADERS: [&str; 13] = [
    "Mark", "Job", "PartWbs", "PartLoc", "PartQty", "PartUom", "Matl", "MatlWbs" , "MatlQty", "MatlUom", "MatlLoc", "Plant", "Program"
];
const SKIP_LOCS: [Option<&str>; 2] = [None, Some("R&D")];
const DELIM: u8 = b'\t';

// lazy static globals that are non-const
lazy_static! {
    // not in SAP to match against material master
    static ref NOT_IN_SAP: Regex = Regex::new(r"^NO[\d\s\w]+SAP$").expect("failed to build regex");

    // for WBS Element validation
    static ref VALID_WBS:  Regex = Regex::new(r"D-\d{7}-\d{5}").expect("failed to build regex");

    // .ready file reader/writer factories
    // TODO: refactor into struct, with convenience (from_path) methods
    static ref READY_READER: ReaderBuilder = {
        let mut reader = ReaderBuilder::new();
        reader
            .delimiter(DELIM);

        reader
    };
    static ref READY_WRITER: WriterBuilder = {
        let mut writer = WriterBuilder::new();
        writer
            .has_headers(false)
            .delimiter(DELIM);

        writer
    };
}

/// Production file processor
/// 
/// Holds reader and writer builders
#[derive(Debug, Default)]
pub struct ProdFileProcessor {
    dry_run: bool,
}

impl ProdFileProcessor {
    /// Create new reader/writer builders
    pub fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }

    /// Process all files in [`CNF_FILES`]
    /// 
    /// [`CNF_FILES`]: `static@super::paths::CNF_FILES`
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

    pub fn process_file(&self, filepath: &PathBuf) -> Result<(), Error> {
        //! Modifications:
        //! - Plant 3 material to RAW
        //! - Skip items for material not in SAP
        //! - Issue items without a valid WBS Element
        //! - Issue Non-production pieces
        //! - SAP part name if different from SN

        // TODO: use MRP name in sigmanest database

        debug!("Processing file {:?}", filepath);

        let mut reader = READY_READER.from_path(filepath)?;
        reader.set_headers( StringRecord::from(HEADERS.to_vec()) );
        
        let results = reader.deserialize::<CnfFileRow>();

        // TODO: find a way to check if file is empty
        {

            let mut prod_writer = READY_WRITER.from_path( filepath.production_file().as_path() )?;
            let mut issue_writer = READY_WRITER.from_path( filepath.issue_file().as_path() )?;

            for result in results {
                trace!("{:?}", result);
                // TODO: log errors
                let mut record = result.expect("Failed to deserialize row");

                // filter out items based on material location
                if SKIP_LOCS[..].contains(&record.matl_loc.as_deref()) {
                    debug!("Skipping due to location: {:?}", &record.matl_loc);
                    continue;
                }
    
                // consume all HS02 material from RAW
                if record.plant == Plant::Williamsport {
                    debug!("Williamsport record; changing location to RAW");
                    record.matl_loc = Some("RAW".into());
                }
    
                if VALID_WBS.is_match(&record.part_wbs) {
                    debug!("Valid WBS element: {}", &record.part_wbs);

                    // write new file with changes
                    prod_writer.serialize(record)?;
                    prod_writer.flush()?;
                } else {
                    debug!("Invalid WBS element: {}", &record.part_wbs);

                    // send to issue file;
                    issue_writer.serialize::<IssueFileRow>(record.into())?;
                    issue_writer.flush()?;
                }
            }
        }

        if !self.dry_run {
            // move file to backup
            let backup = filepath.backup_file();
            debug!("moving file to {:?}", backup);

            std::fs::copy(filepath, backup.as_path()).expect("failed to backup file");
            // std::fs::remove_file(filepath).expect("failed to remove original file");
        }
    
        Ok(())
    }
}
