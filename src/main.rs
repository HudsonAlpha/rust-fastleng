
extern crate clap;
extern crate env_logger;
extern crate exitcode;
extern crate log;

use clap::{App, Arg, value_t};
use log::{error, info};
use std::collections::BTreeMap;
use std::fs::File;

use fastleng::fastx_loader::gather_fastx_stats;
use fastleng::length_stats::{compute_length_stats, LengthStats};

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    //initialize logging for our benefit later
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let matches = App::new("fastleng")
        .version(VERSION.unwrap_or("?"))
        .author("J. Matthew Holt <jholt@hudsonalpha.org>")
        .about("fastleng - a sequence length statistics generator for fastx files")
        .arg(
            Arg::with_name("out_json")
            .short("o")
            .long("--out-json")
            .takes_value(true)
            .help("The output statistics json (default: stdout)")
        )
        .arg(
            Arg::with_name("FASTX")
                .help("The FASTQ/A file to gather stats on, gzip accepted")
                .required(true)
                .index(1)
        )
        .get_matches();

    let fastx_fn: String = matches.value_of("FASTX").unwrap().to_string();
    let out_fn: String = value_t!(matches.value_of("out_json"), String).unwrap_or_else(|_| "stdout".to_string());

    info!("Input parameters (required):");
    info!("\tFASTX: \"{}\"", fastx_fn);

    match File::open(&fastx_fn) {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to open FASTX file: {:?}", fastx_fn);
            error!("Error: {:?}", e);
            std::process::exit(exitcode::NOINPUT);
        }
    };

    if out_fn != "stdout" {
        match File::create(&out_fn) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to create output JSON file: {:?}", out_fn);
                error!("Error: {:?}", e);
                std::process::exit(exitcode::NOINPUT);
            }
        };
    }

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

    //this is what we should put in the file
    if out_fn == "stdout" {
        let pretty_json: String = serde_json::to_string_pretty(&length_metrics).unwrap();
        println!("{}", pretty_json);
    }
    else {
        info!("Saving results to file: {}", out_fn);
        let out_file = match File::create(&out_fn) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to create output JSON file: {:?}", out_fn);
                error!("Error: {:?}", e);
                std::process::exit(exitcode::NOINPUT);
            }
        };
        serde_json::to_writer_pretty(out_file, &length_metrics).unwrap();
    }

    info!("Processes successfully finished.")
}
