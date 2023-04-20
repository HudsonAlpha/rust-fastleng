
use log::{info, warn};
use rust_htslib::{bam, bam::Read};
use std::collections::BTreeMap;

/// This is the main function for gathering all sequence lengths for a fastx file into a BTreeMap.
/// # Arguments
/// * `filename` - the filename to read sequences from
/// # Examples
/// ```
/// use std::collections::BTreeMap;
/// use fastleng::bam_loader::gather_bam_stats;
/// let filename = "./test_data/single_string.sam";
/// let counts: BTreeMap<usize, u64> = gather_bam_stats(&filename).unwrap();
/// ```
pub fn gather_bam_stats(filename: &str) -> Result<BTreeMap<usize, u64>, Box<dyn std::error::Error>> {
    gather_bam_stats_with_seed(filename, None)
}

/// This will gather sequence lengths from a filename and add them to a provided BTreeMap (`initial_counts`).
/// # Arguments
/// * `filename` - the filename to read sequences from
/// * `initial_counts` - if provided, this will use that BTreeMap as the inital counts, otherwise it will create an empty one
/// # Examples
/// ```
/// use std::collections::BTreeMap;
/// use fastleng::bam_loader::gather_bam_stats_with_seed;
/// let filename = "./test_data/single_string.sam";
/// let initial_counts: BTreeMap<usize, u64> = BTreeMap::new();
/// let counts: BTreeMap<usize, u64> = gather_bam_stats_with_seed(&filename, Some(initial_counts)).unwrap();
/// ```
pub fn gather_bam_stats_with_seed(filename: &str, initial_counts: Option<BTreeMap<usize, u64>>) -> Result<BTreeMap<usize, u64>, Box<dyn std::error::Error>> {
    //create an empty stats file (or use initial counts) and ready the reader
    let mut hash_stats: BTreeMap<usize, u64> = match initial_counts {
        Some(ic) => ic,
        None => BTreeMap::new()
    };
    let mut reader = bam::Reader::from_path(filename)?;

    //go through all the records
    let mut warning_triggered = false;
    let mut count: usize = 0;
    info!("Loading file \"{}\"...", filename);
    for read_entry in reader.records() {
        //all we care about is the sequence length
        let record = read_entry?;
        let seq_len: usize = record.seq_len();

        if !warning_triggered && !record.is_unmapped() {
            // user gave us an aligned file, spit out a one-time warning
            warn!("Detected aligned reads, this is not properly handled: {filename}");
            warning_triggered = true;
        }
        
        //insert 0 if absent; then increment
        let len_count: &mut u64 = hash_stats.entry(seq_len).or_insert(0);
        *len_count += 1;
        
        count += 1;
        if count % 1000000 == 0 {
            info!("Processed {} sequences", count);
        }
    }
    info!("Finished loading file with {} sequences.", count);

    //return the full count list now
    Ok(hash_stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    // allows us to test a bunch at once
    use crate::fastx_loader::gather_multifastx_stats;
    
    /// This one is a single sequence "A"
    fn stats_basic_bam() -> BTreeMap<usize, u64> {
        let mut results: BTreeMap<usize, u64> = BTreeMap::new();
        results.insert(1, 1);
        results
    }

    /// one of each length from 1-5
    fn stats_basic_bam2() -> BTreeMap<usize, u64> {
        let mut results: BTreeMap<usize, u64> = BTreeMap::new();
        for l in 1..6 {
            results.insert(l, 1);
        }
        results
    }

    /// mix of a few lengths from 1-4
    fn stats_basic_bam3() -> BTreeMap<usize, u64> {
        let mut results: BTreeMap<usize, u64> = BTreeMap::new();
        results.insert(1, 3);
        results.insert(2, 2);
        results.insert(3, 1);
        results.insert(4, 2);
        results
    }

    /// some longer strings
    fn stats_basic_bam4() -> BTreeMap<usize, u64> {
        let mut results: BTreeMap<usize, u64> = BTreeMap::new();
        results.insert(50, 2);
        results.insert(100, 2);
        results.insert(150, 2);
        results.insert(1000, 1);
        results
    }

    #[test]
    fn test_basic_sam() {
        //build some inputs
        let filename = "./test_data/single_string.sam";

        //get the expected outputs
        let expected = stats_basic_bam();

        //now do it for real
        let hash_stats = gather_bam_stats(&filename).unwrap();
        assert_eq!(hash_stats, expected);
    }

    #[test]
    fn test_basic_sam2() {
        //build some inputs
        let filename = "./test_data/five_strings.sam";

        //get the expected outputs
        let expected = stats_basic_bam2();

        //now do it for real
        let hash_stats = gather_bam_stats(&filename).unwrap();
        assert_eq!(hash_stats, expected);
    }

    #[test]
    fn test_basic_sam3() {
        //build some inputs
        let filename = "./test_data/small_strings.sam";

        //get the expected outputs
        let expected = stats_basic_bam3();

        //now do it for real
        let hash_stats = gather_bam_stats(&filename).unwrap();
        assert_eq!(hash_stats, expected);
    }

    #[test]
    fn test_basic_sam4() {
        //build some inputs
        let filename = "./test_data/long_strings.sam";

        //get the expected outputs
        let expected = stats_basic_bam4();

        //now do it for real
        let hash_stats = gather_bam_stats(&filename).unwrap();
        assert_eq!(hash_stats, expected);
    }

    #[test]
    fn test_basic_bam4() {
        //build some inputs
        let filename = "./test_data/long_strings.bam";

        //get the expected outputs
        let expected = stats_basic_bam4();

        //now do it for real
        let hash_stats = gather_bam_stats(&filename).unwrap();
        assert_eq!(hash_stats, expected);
    }

    #[test]
    #[should_panic]
    fn test_error_handling() {
        let filename = "./test_data/panic_file.fa";
        let _hash_stats = gather_bam_stats(&filename).unwrap();
    }

    #[test]
    fn test_multifastx() {
        let filenames = [
            "./test_data/single_string.sam",
            "./test_data/five_strings.sam",
            "./test_data/small_strings.sam",
            "./test_data/long_strings.bam"
        ];

        //get the expected outputs
        let expected_list = [
            stats_basic_bam(),
            stats_basic_bam2(),
            stats_basic_bam3(),
            stats_basic_bam4()
        ];

        //sum the expected outputs
        let mut expected: BTreeMap<usize, u64> = BTreeMap::new();
        for results in expected_list.iter() {
            for (key, value) in results.iter() {
                let len_count: &mut u64 = expected.entry(*key).or_insert(0);
                *len_count += value;
            }
        }

        //now do it for real
        let hash_stats = gather_multifastx_stats(&filenames).unwrap();
        assert_eq!(hash_stats, expected);
    }

    #[test]
    #[should_panic]
    fn test_multifastx_error_handling() {
        let filenames = [
            "./test_data/single_string.bam",
            "./test_data/panic_file.fa"
        ];
        let _hash_stats = gather_multifastx_stats(&filenames).unwrap();
    }
}