use fern::{Dispatch, InitError, log_file};
use log::LevelFilter;
use std::io;

use crate::settings::Settings;

pub fn init(conf: &Settings) -> Result<(), InitError> {
    let mut base_config = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(Dispatch::new()
            .level(log_level(&conf.logging.console_log_level))
            .chain(io::stdout())
        );
    if conf.logging.log_to_file {
        base_config = base_config.chain(Dispatch::new()
            .level(log_level(&conf.logging.file_log_level))
            .chain(log_file(&conf.logging.file_log_path)?)
        );
    }
    base_config.apply()?;
    Ok(())
}

fn log_level(level: &String) -> LevelFilter {
    match level.as_str() {
        "trace" => LevelFilter::Trace,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    }
}
