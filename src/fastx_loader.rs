
extern crate needletail;

use needletail::parse_fastx_file;
use std::collections::HashMap;

/// This is the main function for gathering all sequence lengths for a fastx file into a HashMap.
/// # Examples
/// ```ignore
/// use std::collections::HashMap;
/// use fastleng::fastx_loader::gather_fastx_stats;
/// let filename = "/path/to/file.fq.gz";
/// let counts: HashMap<usize, u64> = gather_fastx_stats(&filename);
/// ```
pub fn gather_fastx_stats(filename: &str) -> HashMap<usize, u64> {
    //create an empty stats file and ready the reader
    let mut hash_stats: HashMap<usize, u64> = HashMap::new();
    let mut reader = parse_fastx_file(&filename).expect("valid fastx path/file");

    //go through all the records
    while let Some(record) = reader.next() {
        //all we care about is the sequence length
        let seq_rec = record.expect("invalid record");
        let seq_len: usize = seq_rec.num_bases();
        
        //insert 0 if absent; then increment
        let len_count: &mut u64 = hash_stats.entry(seq_len).or_insert(0);
        *len_count += 1;
    }

    //return the full count list now
    hash_stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{Builder, NamedTempFile};
    use std::fs::File;
    use std::io::Write;
    
    /// This one is a single sequence "A"
    fn create_basic_fasta() -> NamedTempFile {
        let temp_file: NamedTempFile = Builder::new().prefix("temp_data_").suffix(".fa").tempfile().unwrap();
        let filename: String = temp_file.path().to_str().unwrap().to_string();

        let mut fasta_file: File = File::create(filename).unwrap();
        let data_string = b">1
A";
        fasta_file.write_all(data_string).unwrap();
        fasta_file.flush().unwrap();

        temp_file
    }

    fn stats_basic_fasta() -> HashMap<usize, u64> {
        let mut results: HashMap<usize, u64> = HashMap::new();
        results.insert(1, 1);
        results
    }

    /// one of each length from 1-5
    fn create_basic_fasta2() -> NamedTempFile {
        let temp_file: NamedTempFile = Builder::new().prefix("temp_data_").suffix(".fa").tempfile().unwrap();
        let filename: String = temp_file.path().to_str().unwrap().to_string();

        let mut fasta_file: File = File::create(filename).unwrap();
        let data_string = b">1
A
>2
AA
>3
AAA
>4
AAAA
>5
AAAAA";
        fasta_file.write_all(data_string).unwrap();
        fasta_file.flush().unwrap();

        temp_file
    }

    fn stats_basic_fasta2() -> HashMap<usize, u64> {
        let mut results: HashMap<usize, u64> = HashMap::new();
        results.insert(1, 1);
        results.insert(2, 1);
        results.insert(3, 1);
        results.insert(4, 1);
        results.insert(5, 1);
        results
    }

    /// mix of a few lengths from 1-4
    fn create_basic_fasta3() -> NamedTempFile {
        let temp_file: NamedTempFile = Builder::new().prefix("temp_data_").suffix(".fa").tempfile().unwrap();
        let filename: String = temp_file.path().to_str().unwrap().to_string();

        let mut fasta_file: File = File::create(filename).unwrap();
        let data_string = b">1
AAAA
>2
AA
>3
A
>4
AA
>5
A
>6
AAA
>7
AAAA
>8
A";
        fasta_file.write_all(data_string).unwrap();
        fasta_file.flush().unwrap();

        temp_file
    }

    fn stats_basic_fasta3() -> HashMap<usize, u64> {
        let mut results: HashMap<usize, u64> = HashMap::new();
        results.insert(1, 3);
        results.insert(2, 2);
        results.insert(3, 1);
        results.insert(4, 2);
        results
    }

    #[test]
    fn test_basic_fasta() {
        //build some inputs
        let temp_file = create_basic_fasta();
        let filename = temp_file.path().to_str().unwrap().to_string();

        //get the expected outputs
        let expected = stats_basic_fasta();

        //now do it for real
        let hash_stats = gather_fastx_stats(&filename);
        assert_eq!(hash_stats, expected);
    }

    #[test]
    fn test_basic_fasta2() {
        //build some inputs
        let temp_file = create_basic_fasta2();
        let filename = temp_file.path().to_str().unwrap().to_string();

        //get the expected outputs
        let expected = stats_basic_fasta2();

        //now do it for real
        let hash_stats = gather_fastx_stats(&filename);
        assert_eq!(hash_stats, expected);
    }

    #[test]
    fn test_basic_fasta3() {
        //build some inputs
        let temp_file = create_basic_fasta3();
        let filename = temp_file.path().to_str().unwrap().to_string();

        //get the expected outputs
        let expected = stats_basic_fasta3();

        //now do it for real
        let hash_stats = gather_fastx_stats(&filename);
        assert_eq!(hash_stats, expected);
    }
}