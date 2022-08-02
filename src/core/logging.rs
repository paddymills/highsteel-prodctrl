
//! logging frameworks

use simplelog::{LevelFilter, Config, WriteLogger};

use std::fs::File;
// use std::io::stderr;

pub fn init_logger(app: &str) {
    // TODO: add switch to log to stderr

    let log_name = format!(r"test\{}.log", app);

    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(log_name).expect("failed to create log")
        // stderr()
    ).expect("Failed to init logger");
}
