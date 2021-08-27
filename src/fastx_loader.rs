
extern crate log;
extern crate needletail;

use log::info;
use needletail::parse_fastx_file;
use std::collections::BTreeMap;

/// This is the main function for gathering all sequence lengths for a fastx file into a BTreeMap.
/// # Examples
/// ```
/// use std::collections::BTreeMap;
/// use fastleng::fastx_loader::gather_fastx_stats;
/// let filename = "./test_data/single_string.fa";
/// let counts: BTreeMap<usize, u64> = gather_fastx_stats(&filename).unwrap();
/// ```
pub fn gather_fastx_stats(filename: &str) -> Result<BTreeMap<usize, u64>, Box<dyn std::error::Error>> {
    //create an empty stats file and ready the reader
    let mut hash_stats: BTreeMap<usize, u64> = BTreeMap::new();
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
}