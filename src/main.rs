
//use std::path::PathBuf;

//extern crate needletail;
//use needletail::parse_fastx_file;

use std::collections::BTreeMap;

use fastleng::fastx_loader::gather_fastx_stats;
use fastleng::length_stats::{LengthStats,compute_length_stats};

fn main() {
    //let test_file = "/Users/matt/Downloads/test_1M.fq.gz";
    let test_file = "/Users/matt/Downloads/test_100k_pb.fq.gz";
    
    //load the fastx file lengths
    let length_counts: BTreeMap<usize, u64> = gather_fastx_stats(&test_file);
    
    //compute the stats
    let length_metrics: LengthStats = compute_length_stats(&length_counts);
    println!("{:?}", length_metrics);

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&length_metrics).unwrap();
    println!("{}", j);
}
