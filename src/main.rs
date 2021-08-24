
extern crate clap;
extern crate env_logger;
extern crate exitcode;
extern crate log;

use clap::{App, Arg};
use log::{info, error};
use std::collections::BTreeMap;
use std::fs::File;

use fastleng::fastx_loader::gather_fastx_stats;
use fastleng::length_stats::{LengthStats,compute_length_stats};

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    //initialize logging for our benefit later
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let matches = App::new("fastleng")
        .version(VERSION.unwrap_or("?"))
        .author("J. Matthew Holt <jholt@hudsonalpha.org>")
        .about("fastleng - a sequence length statistics generator for fastx files")
        .arg(Arg::with_name("FASTX")
            .help("The FASTQ/A file to gather stats on, gzip accepted")
            .required(true)
            .index(1))
        .get_matches();
    
    let fastx_fn: String = matches.value_of("FASTX").unwrap().to_string();

    info!("Input parameters (required):");
    info!("\tFASTX: \"{}\"", fastx_fn);
    match File::open(&fastx_fn) {
        Ok(_) => {},
        Err(e) => {
            error!("Failed to open FASTX file: {:?}", fastx_fn);
            error!("Error: {:?}", e);
            std::process::exit(exitcode::NOINPUT);
        }
    };

    //load the fastx file lengths
    let length_counts: BTreeMap<usize, u64> = match gather_fastx_stats(&fastx_fn) {
        Ok(result) => result,
        Err(e) => {
            error!("Error while parsing FASTX file: {:?}", fastx_fn);
            error!("Error: {:?}", e);
            std::process::exit(exitcode::IOERR);
        }
    };
    
    //compute the stats
    let length_metrics: LengthStats = compute_length_stats(&length_counts);
    // Serialize it to a JSON string.
    let json_format: String = serde_json::to_string(&length_metrics).unwrap();
    info!("Length metrics: {}", json_format);
    
    //TODO: this is what we should put in the file
    //let pretty_json: String = serde_json::to_string_pretty(&length_metrics).unwrap();
    
}
