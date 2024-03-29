
//! filesystem modules

use std::path::Path;

/// Check if file is is am empty text file.
/// 
/// This is done by checking the file's metadata, figuring a `0kb` text file
/// is an empty file. Getting the [`metadata`] can fail, so in the case
/// of failure, this will return `false`. Considering that getting [`metadata`]
/// should only fail due to permissions or a non-existant path, this shouldn't
/// be an issue
/// 
/// [`metadata`]: std::fs::metadata
pub fn is_empty_file<P>(filepath: P) -> bool
    where P: AsRef<Path>
{
    match std::fs::metadata(filepath) {
        Ok(metadata) => metadata.len() == 0,
        _ => false
    }
}

/// Create a filename with a naturally sortable timestamp
pub fn timestamped_file(prefix: &str, ext: &str) -> String {
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();

    format!("{}_{}.{}", prefix, timestamp, ext)
}
