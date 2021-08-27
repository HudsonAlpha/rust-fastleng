
extern crate log;
extern crate needletail;

use log::{error, info};
use needletail::parse_fastx_file;
use std::collections::BTreeMap;

/// This is the main function for gathering all sequence lengths for a fastx file into a BTreeMap.
/// # Arguments
/// * `filename` - the filename to read sequences from
/// # Examples
/// ```
/// use std::collections::BTreeMap;
/// use fastleng::fastx_loader::gather_fastx_stats;
/// let filename = "./test_data/single_string.fa";
/// let counts: BTreeMap<usize, u64> = gather_fastx_stats(&filename).unwrap();
/// ```
pub fn gather_fastx_stats(filename: &str) -> Result<BTreeMap<usize, u64>, Box<dyn std::error::Error>> {
    gather_fastx_stats_with_seed(filename, None)
}

/// This will gather sequence lengths from a filename and add them to a provided BTreeMap (`initial_counts`).
/// # Arguments
/// * `filename` - the filename to read sequences from
/// * `initial_counts` - if provided, this will use that BTreeMap as the inital counts, otherwise it will create an empty one
/// # Examples
/// ```
/// use std::collections::BTreeMap;
/// use fastleng::fastx_loader::gather_fastx_stats_with_seed;
/// let filename = "./test_data/single_string.fa";
/// let initial_counts: BTreeMap<usize, u64> = BTreeMap::new();
/// let counts: BTreeMap<usize, u64> = gather_fastx_stats_with_seed(&filename, Some(initial_counts)).unwrap();
/// ```
pub fn gather_fastx_stats_with_seed(filename: &str, initial_counts: Option<BTreeMap<usize, u64>>) -> Result<BTreeMap<usize, u64>, Box<dyn std::error::Error>> {
    //create an empty stats file (or use initial counts) and ready the reader
    let mut hash_stats: BTreeMap<usize, u64> = match initial_counts {
        Some(ic) => ic,
        None => BTreeMap::new()
    };
    let mut reader = parse_fastx_file(&filename)?;

    //go through all the records
    let mut count: usize = 0;
    info!("Loading file \"{}\"...", filename);
    while let Some(record) = reader.next() {
        //all we care about is the sequence length
        let seq_rec = record?;
        let seq_len: usize = seq_rec.num_bases();
        
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

/// This will iterate through multiple fastx files and gather the lengths into a single BTreeMap.
/// # Arguments
/// * `filenames` - the filenames to read sequences from
/// # Examples
/// ```
/// use std::collections::BTreeMap;
/// use fastleng::fastx_loader::gather_multifastx_stats;
/// let filenames = [
///     "./test_data/single_string.fa",
///     "./test_data/five_strings.fa"
/// ];
/// let counts: BTreeMap<usize, u64> = gather_multifastx_stats(&filenames).unwrap();
/// ```
pub fn gather_multifastx_stats<T: AsRef<str> + std::fmt::Debug>(filenames: &[T]) -> Result<BTreeMap<usize, u64>, Box<dyn std::error::Error>> {
    /*
    Notes on the T here: we need to be able to reference as a &str and run the debug formatting for output.
    The above allows us to pass lists/vecs of Strings/&strs without having to do a bunch of work.
    Derived from: https://stackoverflow.com/questions/32723794/how-do-i-write-a-function-that-takes-both-owned-and-non-owned-string-collections/32724666#32724666
    */
    let mut hash_stats: BTreeMap<usize, u64> = BTreeMap::new();
    for filename in filenames.iter() {
        hash_stats = match gather_fastx_stats_with_seed(filename.as_ref(), Some(hash_stats)) {
            Ok(result) => result,
            Err(e) => {
                error!("Error while parsing FASTX file: {:?}", filename);
                error!("Error: {:?}", e);
                return Err(e);
            }
        };
    }
    Ok(hash_stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// This one is a single sequence "A"
    fn stats_basic_fasta() -> BTreeMap<usize, u64> {
        let mut results: BTreeMap<usize, u64> = BTreeMap::new();
        results.insert(1, 1);
        results
    }

    /// one of each length from 1-5
    fn stats_basic_fasta2() -> BTreeMap<usize, u64> {
        let mut results: BTreeMap<usize, u64> = BTreeMap::new();
        for l in 1..6 {
            results.insert(l, 1);
        }
        results
    }

    /// mix of a few lengths from 1-4
    fn stats_basic_fasta3() -> BTreeMap<usize, u64> {
        let mut results: BTreeMap<usize, u64> = BTreeMap::new();
        results.insert(1, 3);
        results.insert(2, 2);
        results.insert(3, 1);
        results.insert(4, 2);
        results
    }

    /// some longer strings
    fn stats_basic_fasta4() -> BTreeMap<usize, u64> {
        let mut results: BTreeMap<usize, u64> = BTreeMap::new();
        results.insert(50, 2);
        results.insert(100, 2);
        results.insert(150, 2);
        results.insert(1000, 1);
        results
    }

    #[test]
    fn test_basic_fasta() {
        //build some inputs
        let filename = "./test_data/single_string.fa";

        //get the expected outputs
        let expected = stats_basic_fasta();

        //now do it for real
        let hash_stats = gather_fastx_stats(&filename).unwrap();
        assert_eq!(hash_stats, expected);
    }

    #[test]
    fn test_basic_fasta2() {
        //build some inputs
        let filename = "./test_data/five_strings.fa";

        //get the expected outputs
        let expected = stats_basic_fasta2();

        //now do it for real
        let hash_stats = gather_fastx_stats(&filename).unwrap();
        assert_eq!(hash_stats, expected);
    }

    #[test]
    fn test_basic_fasta3() {
        //build some inputs
        let filename = "./test_data/small_strings.fa";

        //get the expected outputs
        let expected = stats_basic_fasta3();

        //now do it for real
        let hash_stats = gather_fastx_stats(&filename).unwrap();
        assert_eq!(hash_stats, expected);
    }

    #[test]
    fn test_basic_fasta4() {
        //build some inputs
        let filename = "./test_data/long_strings.fa";

        //get the expected outputs
        let expected = stats_basic_fasta4();

        //now do it for real
        let hash_stats = gather_fastx_stats(&filename).unwrap();
        assert_eq!(hash_stats, expected);
    }

    #[test]
    #[should_panic]
    fn test_error_handling() {
        let filename = "./test_data/panic_file.fa";
        let _hash_stats = gather_fastx_stats(&filename).unwrap();
    }

    #[test]
    fn test_multifastx() {
        let filenames = [
            "./test_data/single_string.fa",
            "./test_data/five_strings.fa",
            "./test_data/small_strings.fa",
            "./test_data/long_strings.fa"
        ];

        //get the expected outputs
        let expected_list = [
            stats_basic_fasta(),
            stats_basic_fasta2(),
            stats_basic_fasta3(),
            stats_basic_fasta4()
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
            "./test_data/single_string.fa",
            "./test_data/panic_file.fa"
        ];
        let _hash_stats = gather_multifastx_stats(&filenames).unwrap();
    }
}