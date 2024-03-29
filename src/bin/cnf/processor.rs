
use csv::{ReaderBuilder, StringRecord, WriterBuilder};

use linya::Progress;
use rayon::prelude::*;
use tokio::sync::mpsc::Sender;

use regex::Regex;
use std::ffi::OsStr;
use std::{
    fs,
    io,
    error::Error,
    path::PathBuf,
    sync::Mutex
};

use crate::api::{CnfFileRow, IssueFileRow, CnfLogRecord};
use crate::paths::*;

use prodctrl::fs::is_empty_file;

const HEADERS: [&str; 13] = [
    "Mark", "Job", "PartWbs", "PartLoc", "PartQty", "PartUom", "Matl", "MatlWbs" , "MatlQty", "MatlUom", "MatlLoc", "Plant", "Program"
];
const SKIP_LOCS: [Option<&str>; 2] = [None, Some("R&D")];
const DELIM: u8 = b'\t';

// lazy static globals that are non-const
lazy_static! {
    // not in SAP to match against material master
    static ref NOT_IN_SAP: Regex = Regex::new(r"^NO[\d\s\w]*SAP$").expect("failed to build regex");

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
#[derive(Debug)]
pub struct ProdFileProcessor {
    dry_run: bool,
    db_queue: Sender<CnfLogRecord>
}

impl ProdFileProcessor {
    /// Create new reader/writer builders
    pub fn new(dry_run: bool, sender: Sender<CnfLogRecord>) -> Self {
        Self { dry_run, db_queue: sender }
    }

    /// Process all files in [`CNF_FILES`]
    /// 
    /// [`CNF_FILES`]: `static@super::paths::CNF_FILES`
    pub fn process_files(&self) -> Result<(), io::Error> {
        let files = get_ready_files()?;

        if files.len() > 0 {
            let progress = Mutex::new( Progress::new() );
            
            let bar = {
                let mut prog = progress.lock().unwrap();
                let bar = prog.bar(files.len(), "Reading files");
                prog.draw(&bar);
    
                bar
            };
    
    
            files.into_par_iter().for_each(|file| {
                if let Err(e) = self.process_file(&file) {
                    error!("Failed to parse file: {:?}", e);

                    self.revert_output(&file);
                }
    
                progress.lock().unwrap().inc_and_draw(&bar, 1);
            });
        }

        Ok(())
    }

    pub fn process_file(&self, filepath: &PathBuf) -> Result<(), Box<dyn Error>> {
        //! Modifications:
        //! - Plant 3 material to RAW
        //! - Skip items for material not in SAP
        //! - Issue items without a valid WBS Element
        //! - Issue Non-production pieces
        //! - SAP part name if different from SN

        // TODO: use MRP name in sigmanest database

        // check if file is empty
        if is_empty_file(filepath) {
            info!("Skipping empty file {:?}", filepath);

            // TODO: refactor (same code as after processing)
            let backup = filepath.backup_file();
            fs::copy(filepath, backup).expect("failed to backup file");
            fs::remove_file(filepath).expect("failed to remove original file");

            return Ok(())
        }

        info!("Processing file {:?}", filepath);

        let mut reader = READY_READER.from_path(filepath)?;
        reader.set_headers( StringRecord::from(HEADERS.to_vec()) );
        
        let results = reader.deserialize::<CnfFileRow>();

        let out_prod_file = filepath.production_file();
        let out_issue_file = filepath.issue_file();

        {
            let mut prod_writer = READY_WRITER.from_path( out_prod_file.as_path() )?;
            let mut issue_writer = READY_WRITER.from_path( out_issue_file.as_path() )?;
            
            for result in results {
                trace!("{:?}", result);
                let record = result.map_err(|e| error!("Failed to deserialize row: {}", e)).unwrap();

                // filter out items based on material location
                if SKIP_LOCS[..].contains(&record.matl_loc.as_deref()) {
                    debug!("Skipping due to location: {:?}", &record.matl_loc);
                    continue;
                }

                // filter out items not in sap
                if NOT_IN_SAP.is_match(&record.matl) {
                    debug!("Skipping due to SAP MM: {:?}", &record.matl);
                    continue;
                }
    
                // consume all HS02 material from RAW
                // if record.plant == Plant::Williamsport {
                //     debug!("Williamsport record; changing location to RAW");
                //     record.matl_loc = Some("RAW".into());
                // }
    
                if VALID_WBS.is_match(&record.part_wbs) {
                    debug!("Valid WBS element: {}", &record.part_wbs);

                    // log row to db, ignoring failure because this is not mission critical
                    let _ = self.db_queue.blocking_send(CnfLogRecord::new(&record, filepath));

                    // write new file with changes
                    prod_writer.serialize(record)?;
                } else {
                    debug!("Invalid WBS element: {}", &record.part_wbs);
                    
                    let issue_record = record.try_into()?;

                    // send to issue file;
                    issue_writer.serialize::<IssueFileRow>(issue_record)?;
                }
            }

            // flush writer buffers
            prod_writer.flush()?;
            issue_writer.flush()?;
        }

        // cleanup empty files
        // files are created (regardless of use) at Writer creation
        for file in [&out_prod_file, &out_issue_file] {
            if is_empty_file(file) {
                // failure is not critical, so ignore any errors
                let _ = fs::remove_file(file);
            }
        }

        if !self.dry_run {
            // move original file to backup
            let backup = filepath.backup_file();
            debug!("moving file to {:?}", backup);
            fs::copy(filepath, backup).expect("failed to backup file");
            fs::remove_file(filepath).expect("failed to remove original file");

            // archive processed files
            for file in [&out_prod_file, &out_issue_file] {
                // ignore failure since it is most likely due to
                // a file not existing (empty file was already cleaned up)
                if let Ok(_) = fs::copy(file, file.archive_file()) {
                    debug!("archived processed file: {:?}", file);
                }
            }
        }
    
        Ok(())
    }

    pub fn log_processed(&self, existing_files: Vec<String>) -> Result<(), Box<dyn Error>> {
        let files = std::fs::read_dir(&*CNF_FILES.join( "processed" ))?
            .filter_map(|f| f.ok())
            .filter(|f| {
                let path = f.path();

                let filename = path.file_name().unwrap_or(OsStr::new("skip file")).to_str().unwrap_or("skip file");     // has extension
                let filestem = path.file_stem().unwrap_or(OsStr::new("not_a_file")).to_str().unwrap_or("invalid utf8"); // does not have extension
                
                PROD_FILE_NAME.is_match(filename) || !existing_files.contains(&filestem.into())
            })
            .map(|f| f.path().to_path_buf())
            .collect::<Vec<PathBuf>>();

        if files.len() > 0 {
            let progress = Mutex::new( Progress::new() );
            
            let bar = {
                let mut prog = progress.lock().unwrap();
                let bar = prog.bar(files.len(), "Reading files");
                prog.draw(&bar);
    
                bar
            };
    
    
            files.into_par_iter().for_each(|file| {
                let filename = file.file_stem().unwrap_or_default().to_str().unwrap();
                if let Ok(mut reader) = READY_READER.from_path(&file) {
                    reader.set_headers( StringRecord::from(HEADERS.to_vec()) );
                    
                    reader
                        .deserialize::<CnfFileRow>()
                        .filter(|r| r.is_ok())
                        .map(|r| r.unwrap())
                        .for_each(|record| {
                            let _ = self.db_queue.blocking_send(
                                CnfLogRecord {
                                    filename: filename.into(),
                                    record: record.clone()
                                });
                        });
                }
    
                progress.lock().unwrap().inc_and_draw(&bar, 1);
            });
        }

        Ok(())
    }

    fn revert_output(&self, filepath: &PathBuf) {
        fs::remove_file(&filepath.production_file()).expect("Failed to remove failed production file");
        fs::remove_file(&filepath.issue_file()).expect("Failed to remove failed issue file");
    }
}
