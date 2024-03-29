
extern crate serde;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// This will compute the total number of bases and sequences by iterating over the length stats and return a tuple (`total_bases`, `total_seqs`).
/// # Arguments
/// * `length_counts` - a BTreeMap with the sequence length as the key, and the value the total number of sequences with that length
/// # Examples
/// ```
/// use std::collections::BTreeMap;
/// use fastleng::length_stats::compute_total_counts;
/// let length_counts: BTreeMap<usize, u64> = [
///     (5, 10),
///     (10, 3)
/// ].iter().cloned().collect();
/// let (total_bases, total_seqs) = compute_total_counts(&length_counts);
/// assert_eq!(total_bases, 80);
/// assert_eq!(total_seqs, 13);
/// ```
pub fn compute_total_counts(length_counts: &BTreeMap<usize, u64>) -> (u64, u64) {
    let mut total_bases: u64 = 0;
    let mut total_seqs: u64 = 0;
    for (length, count) in length_counts.iter() {
        total_bases += (*length as u64) * count;
        total_seqs += count;
    }
    (total_bases, total_seqs)
}

/// This will compute the median length of the sequences captured by some length statistics.
/// This metric is imprecise for some instances of an even number of sequences (e.g. does not take the mean).
/// # Arguments
/// * `length_counts` - a BTreeMap with the sequence length as the key, and the value the total number of sequences with that length
/// * `total_seqs` - the total number of sequences represented by `length_counts`, this can be computed by `compute_total_counts(...)`
/// # Examples
/// ```
/// use std::collections::BTreeMap;
/// use fastleng::length_stats::{compute_median_length,compute_total_counts};
/// let length_counts: BTreeMap<usize, u64> = [
///     (5, 10),
///     (10, 3)
/// ].iter().cloned().collect();
/// let (_total_bases, total_seqs) = compute_total_counts(&length_counts);
/// let median_length = compute_median_length(&length_counts, total_seqs);
/// assert_eq!(median_length, 5.0);
/// ```
pub fn compute_median_length(length_counts: &BTreeMap<usize, u64>, total_seqs: u64) -> f64 {
    //find the middle index
    let middle_seq_index: u64 = total_seqs / 2;
    let mut total_observed = 0;
    for (seq_len, seq_count) in length_counts.iter() {
        total_observed += seq_count;

        //loop until we observe more than the target index
        if total_observed > middle_seq_index {
            return *seq_len as f64;
        }
    }

    //this case only happens with empty files
    assert!(total_seqs == 0 && length_counts.is_empty());
    0.0
}

/// This will compute the N-score (e.g. N50) for the sequence lengths provided. 
/// For details on this measure, see <https://www.molecularecologist.com/2017/03/29/whats-n50/>.
/// # Arguments
/// * `length_counts` - a BTreeMap with the sequence length as the key, and the value the total number of sequences with that length
/// * `total_bases` - the total number of bases represented by the `length_counts` parameter, this can be computed by `compute_total_counts(...)`
/// * `target` - the score target; e.g. for N50, N75, and N90, this parameter should be 50, 75, and 90 respectively
/// # Examples
/// ```
/// use std::collections::BTreeMap;
/// use fastleng::length_stats::{compute_n_score,compute_total_counts};
/// let length_counts: BTreeMap<usize, u64> = [
///     (5, 10),
///     (10, 3)
/// ].iter().cloned().collect();
/// let (total_bases, _total_seqs) = compute_total_counts(&length_counts);
/// let n50_score = compute_n_score(&length_counts, total_bases, 50);
/// assert_eq!(n50_score, 5);
/// ```
pub fn compute_n_score(length_counts: &BTreeMap<usize, u64>, total_bases: u64, target: usize) -> usize {
    //make sure this is in our allowed range
    assert!((1..=99).contains(&target));

    //calculate the target number of bases
    let target_bases: f64 = (target as u64*total_bases) as f64 / 100.0;
    let mut current_bases: u64 = 0;
    for (seq_len, seq_count) in length_counts.iter().rev() {
        current_bases += (*seq_len as u64) * *seq_count;
        if current_bases as f64 >= target_bases {
            return *seq_len;
        }
    }

    //this only happens with empty files
    assert!(total_bases == 0 && length_counts.is_empty());
    0
}

/// This struct encapsulates the various statistics we return
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LengthStats {
    /// The total number of bases analyzed
    pub total_bases: u64,
    /// The total number of sequences (i.e. strings) analyzed
    pub total_sequences: u64,
    /// The average length of the sequences
    pub mean_length: f64,
    /// The median length of the sequences
    pub median_length: f64,
    /// N10 - 10% of bases are in sequences of length greater than this value
    pub n10: usize,
    /// N25 - 25% of bases are in sequences of length greater than this value
    pub n25: usize,
    /// N50 - 50% of bases are in sequences of length greater than this value
    pub n50: usize,
    /// N75 - 75% of bases are in sequences of length greater than this value
    pub n75: usize,
    /// N90 - 90% of bases are in sequences of length greater than this value
    pub n90: usize
}

/// This will compute multiple different summary statistics based on the length BTreeMap and return a HashMap with all the various metrics
/// # Arguments
/// * `length_counts` - a BTreeMap with the sequence length as the key, and the value the total number of sequences with that length
/// # Examples
/// ```
/// use std::collections::BTreeMap;
/// use fastleng::length_stats::{compute_length_stats,LengthStats};
/// let length_counts: BTreeMap<usize, u64> = [
///     (5, 10),
///     (10, 3)
/// ].iter().cloned().collect();
/// let summary_stats: LengthStats = compute_length_stats(&length_counts);
/// assert_eq!(summary_stats.total_bases, 80);
/// assert_eq!(summary_stats.total_sequences, 13);
/// ```
pub fn compute_length_stats(length_counts: &BTreeMap<usize, u64>) -> LengthStats {
    //first get all the totals
    let (total_bases, total_seqs): (u64, u64) = compute_total_counts(length_counts);
    let median_length: f64 = compute_median_length(length_counts, total_seqs);
    let n10: usize = compute_n_score(length_counts, total_bases, 10);
    let n25: usize = compute_n_score(length_counts, total_bases, 25);
    let n50: usize = compute_n_score(length_counts, total_bases, 50);
    let n75: usize = compute_n_score(length_counts, total_bases, 75);
    let n90: usize = compute_n_score(length_counts, total_bases, 90);

    //now put the composite stats together
    let final_stats: LengthStats = LengthStats {
        total_bases, 
        total_sequences: total_seqs,
        mean_length: (total_bases as f64) / (total_seqs as f64),
        median_length,
        n10,
        n25,
        n50,
        n75,
        n90
    };
    final_stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_total_counts() {
        let seq_lens: BTreeMap<usize, u64> = [
            (10, 100)
        ].iter().cloned().collect();

        let expected_total_bases: u64 = 1000;
        let expected_total_seqs: u64 = 100;

        let computed_results = compute_total_counts(&seq_lens);
        assert_eq!((expected_total_bases, expected_total_seqs), computed_results);
    }

    #[test]
    fn test_compute_median_length() {
        //odd one
        let seq_lens: BTreeMap<usize, u64> = [
            (1, 1),
            (2, 1),
            (3, 1)
        ].iter().cloned().collect();
        let (_total_bases, total_seqs) = compute_total_counts(&seq_lens);
        let median: f64 = compute_median_length(&seq_lens, total_seqs);
        assert_eq!(median, 2.0);

        //even one - this should be the average of index 1 & 2
        let seq_lens: BTreeMap<usize, u64> = [
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1)
        ].iter().cloned().collect();
        let (_total_bases, total_seqs) = compute_total_counts(&seq_lens);
        let median: f64 = compute_median_length(&seq_lens, total_seqs);
        assert_eq!(median, 3.0);

        //even one - but both are in 2
        let seq_lens: BTreeMap<usize, u64> = [
            (1, 1),
            (2, 2),
            (3, 1)
        ].iter().cloned().collect();
        let (_total_bases, total_seqs) = compute_total_counts(&seq_lens);
        let median: f64 = compute_median_length(&seq_lens, total_seqs);
        assert_eq!(median, 2.0);

        //even one - but both are in 2
        let seq_lens: BTreeMap<usize, u64> = [
            (2, 3),
            (3, 2),
            (4, 1)
        ].iter().cloned().collect();
        let (_total_bases, total_seqs) = compute_total_counts(&seq_lens);
        let median: f64 = compute_median_length(&seq_lens, total_seqs);
        assert_eq!(median, 3.0);
    }

    #[test]
    fn test_compute_n_score() {
        let seq_lens: BTreeMap<usize, u64> = [
            (1, 1),
            (2, 1),
            (3, 1)
        ].iter().cloned().collect();
        let (total_bases, _total_seqs) = compute_total_counts(&seq_lens);
        let n_score = compute_n_score(&seq_lens, total_bases, 50);
        assert_eq!(n_score, 3);
        
        let seq_lens: BTreeMap<usize, u64> = [
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1)
        ].iter().cloned().collect();
        let (total_bases, _total_seqs) = compute_total_counts(&seq_lens);
        let n_score = compute_n_score(&seq_lens, total_bases, 50);
        assert_eq!(n_score, 3);

        let seq_lens: BTreeMap<usize, u64> = [
            (1, 1),
            (2, 2),
            (3, 1)
        ].iter().cloned().collect();
        let (total_bases, _total_seqs) = compute_total_counts(&seq_lens);
        let n_score = compute_n_score(&seq_lens, total_bases, 50);
        assert_eq!(n_score, 2);

        let seq_lens: BTreeMap<usize, u64> = [
            (2, 3),
            (3, 2),
            (4, 1)
        ].iter().cloned().collect();
        let (total_bases, _total_seqs) = compute_total_counts(&seq_lens);
        let n_score = compute_n_score(&seq_lens, total_bases, 50);
        assert_eq!(n_score, 3);

        let seq_lens: BTreeMap<usize, u64> = [
            (1, 1000),
            (1000, 1)
        ].iter().cloned().collect();
        let (total_bases, _total_seqs) = compute_total_counts(&seq_lens);
        let n_score = compute_n_score(&seq_lens, total_bases, 50);
        assert_eq!(n_score, 1000);
        let n_score = compute_n_score(&seq_lens, total_bases, 51);
        assert_eq!(n_score, 1);

        let seq_lens: BTreeMap<usize, u64> = [
            (1, 1001),
            (1000, 1)
        ].iter().cloned().collect();
        let (total_bases, _total_seqs) = compute_total_counts(&seq_lens);
        let n_score = compute_n_score(&seq_lens, total_bases, 50);
        assert_eq!(n_score, 1);
        let n_score = compute_n_score(&seq_lens, total_bases, 49);
        assert_eq!(n_score, 1000);

        let mut seq_lens: BTreeMap<usize, u64> = BTreeMap::new();
        for x in 1..101 {
            seq_lens.insert(x, 1);
        }
        let (total_bases, _total_seqs) = compute_total_counts(&seq_lens);
        
        //do a test of all of the different n_values here
        for n_value in 1..100 {
            let n_score = compute_n_score(&seq_lens, total_bases, n_value);
            let target_value = (total_bases as f64) * (n_value as f64 / 100.0);
            let mut total_count: u64 = 0;
            for (seq_len, seq_count) in seq_lens.iter() {
                if *seq_len >= n_score {
                    total_count += (*seq_len as u64) * *seq_count;
                }
            }
            //println!("{} {} {} {}", n_value, n_score, total_count, target_value);
            assert!((total_count as f64) >= target_value);
        }
    }

    #[test]
    fn test_full_all_same() {
        let seq_lens: BTreeMap<usize, u64> = [
            (10, 100)
        ].iter().cloned().collect();

        let expected_stats: LengthStats = LengthStats {
            total_bases: 1000,
            total_sequences: 100,
            mean_length: 10.0,
            median_length: 10.0,
            n10: 10,
            n25: 10,
            n50: 10,
            n75: 10,
            n90: 10
        };

        let actual_stats: LengthStats = compute_length_stats(&seq_lens);
        assert_eq!(expected_stats, actual_stats);
    }
}