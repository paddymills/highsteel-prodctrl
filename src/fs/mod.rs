
//! filesystem modules

pub mod cnf;

use std::path::Path;

/// Check if file is is am empty text file.
/// 
/// This is done by checking the file's metadata, figuring a 0kb text file
/// is an empty file. Getting the [std::fs::metadata] can fail, so in the case
/// of failure, this will return `false`.
pub fn is_empty_file<P>(filepath: P) -> bool
    where P: AsRef<Path>
{
    match std::fs::metadata(filepath) {
        Ok(metadata) => metadata.len() == 0,
        _ => false
    }
}
