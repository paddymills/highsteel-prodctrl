

mod row;
mod processor;

pub use row::*;
pub use processor::ProdFileProcessor;

pub mod paths {
    use regex::Regex;
    use std::{
        io::Error,
        path::PathBuf,
    };

    lazy_static! {
        // paths
        pub static ref CNF_FILES: PathBuf = PathBuf::from(r"\\hssieng\SNData\SimTrans\SAP Data Files\test");

        // regex
        static ref PROD_FILE_NAME: Regex = Regex::new(r"Production_(\d{14}).ready").expect("failed to build regex");
    }

    pub fn get_ready_files() -> Result<Vec<PathBuf>, Error> {
        let files = std::fs::read_dir(&*CNF_FILES)?
            .filter_map(|f| f.ok())
            .filter(|f| PROD_FILE_NAME.is_match(f.file_name().to_str().unwrap_or("skip file")))
            .map(|f| f.path().to_path_buf())
            .collect::<Vec<PathBuf>>();

        Ok(files)
    }

    pub trait CnfFilePaths {
        fn new_prod_file() -> Self;
        fn new_issue_file() -> Self;
        fn archive_file(self: &Self) -> Self;
        fn backup_file(self: &Self) -> Self;
        fn issue_file(self: &Self) -> Self;
    }

    impl CnfFilePaths for PathBuf {
        fn new_prod_file() -> Self {
            let mut path = CNF_FILES.clone();
            path.push( chrono::Local::now().format("Production_%Y%m%d%H%M%S.ready").to_string() );

            path
        }

        fn new_issue_file() -> Self {
            let mut path = CNF_FILES.clone();
            path.push( chrono::Local::now().format("Issue_%Y%m%d%H%M%S.ready").to_string() );

            path
        }
        
        fn archive_file(self: &Self) -> Self {
            let mut path = CNF_FILES.clone();
            path.push("archive");

            // safe to unwrap Option<&OsStr> here
            //  because we will assume whoever consumes this api
            //  is not dumb enough to call it on a folder or path ending in '..'
            path.push(self.file_name().unwrap());
        
            path
        }
        
        fn backup_file(self: &Self) -> Self {
            let mut path = CNF_FILES.clone();
            path.push("backup");

            // safe to unwrap Option<&OsStr> here
            //  because we will assume whoever consumes this api
            //  is not dumb enough to call it on a folder or path ending in '..'
            path.push(self.file_name().unwrap());
        
            path
        }
        
        fn issue_file(self: &Self) -> Self {
            let mut path = CNF_FILES.clone();

            // safe to unwrap Option<&OsStr> and Option<&str> here
            //  because we already checked that it is a file
            path.push( self.file_name().unwrap().to_str().unwrap().replace("Production", "Issue") );
        
            path
        }
    }
}

#[cfg(test)]
mod tests {
    use super::paths::*;
    use std::path::PathBuf;

    #[test]
    fn test_paths() {
        let test_file = PathBuf::from(r"\\hssieng\SNData\SimTrans\SAP Data Files\test\Production_20220105083000.ready");
        assert_eq!(test_file.archive_file(), PathBuf::from(r"\\hssieng\SNData\SimTrans\SAP Data Files\test\archive\Production_20220105083000.ready"));
        assert_eq!(test_file.backup_file(), PathBuf::from(r"\\hssieng\SNData\SimTrans\SAP Data Files\test\backup\Production_20220105083000.ready"));
        assert_eq!(test_file.issue_file(), PathBuf::from(r"\\hssieng\SNData\SimTrans\SAP Data Files\test\Issue_20220105083000.ready"));
    }
}
