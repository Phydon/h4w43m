mod cpu;
mod gpu;
mod memory;
mod utils;

use std::{error::Error, process};

// use crate::cpu::*;
// use crate::gpu::*;
// use crate::memory::*;
use crate::utils::*;

use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
// use indicatif::{ProgressBar, ProgressStyle};
use log::error;
use owo_colors::colored::*;

fn main() -> Result<(), Box<dyn Error>> {
    // handle Ctrl+C
    ctrlc::set_handler(move || {
        println!(
            "{} {} {} {}",
            "Received Ctrl-C!".bold().red(),
            "🤬",
            "Exit program!".bold().red(),
            "☠",
        );
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    // get config dir
    let config_dir = check_create_config_dir().unwrap_or_else(|err| {
        error!("Unable to find or create a config directory: {err}");
        process::exit(1);
    });

    // initialize the logger
    let _logger = Logger::try_with_str("info") // log warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(
            FileSpec::default()
                .directory(&config_dir)
                .suppress_timestamp(),
        ) // change directory for logs, no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Info) // print infos, warnings and errors also to the console
        .start()
        .unwrap();

    Ok(())
}
