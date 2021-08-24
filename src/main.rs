
extern crate clap;

use clap::{App, Arg};
use std::collections::BTreeMap;

use fastleng::fastx_loader::gather_fastx_stats;
use fastleng::length_stats::{LengthStats,compute_length_stats};

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    //let test_file = "/Users/matt/Downloads/test_1M.fq.gz";
    //let test_file = "/Users/matt/Downloads/test_100k_pb.fq.gz";

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

    //load the fastx file lengths
    let length_counts: BTreeMap<usize, u64> = gather_fastx_stats(&fastx_fn);
    
    //compute the stats
    let length_metrics: LengthStats = compute_length_stats(&length_counts);
    // Serialize it to a JSON string.
    let json_format: String = serde_json::to_string_pretty(&length_metrics).unwrap();
    println!("{}", json_format);
}
