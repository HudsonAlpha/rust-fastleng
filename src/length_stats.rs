
use std::collections::{BTreeMap,HashMap};

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
/// # Examples
/// ```
/// use std::collections::BTreeMap;
/// use fastleng::length_stats::compute_median_length;
/// let length_counts: BTreeMap<usize, u64> = [
///     (5, 10),
///     (10, 3)
/// ].iter().cloned().collect();
/// let total_seqs = 13;
/// let median_length = compute_median_length(&length_counts, total_seqs);
/// assert_eq!(median_length, 5.0);
/// ```
pub fn compute_median_length(length_counts: &BTreeMap<usize, u64>, total_seqs: u64) -> f64 {
    let middle_seq_index: u64 = total_seqs / 2;
    let mut total_observed = 0;
    for (key, value) in length_counts.iter() {
        total_observed += value;
        if total_observed > middle_seq_index {
            return *key as f64;
        }
    }
    0.0
}

/// This will compute multiple different summary statistics based on the length BTreeMap and return a HashMap with all the various metrics
/// # Arguments
/// * `length_counts` - a BTreeMap with the sequence length as the key, and the value the total number of sequences with that length
/// # Examples
/// ```
/// use std::collections::{BTreeMap,HashMap};
/// use fastleng::length_stats::compute_length_stats;
/// let length_counts: BTreeMap<usize, u64> = [
///     (5, 10),
///     (10, 3)
/// ].iter().cloned().collect();
/// let summary_stats: HashMap<String, f64> = compute_length_stats(&length_counts);
/// assert_eq!(summary_stats.get("total_bases"), Some(&80.0));
/// assert_eq!(summary_stats.get("total_sequences"), Some(&13.0));
/// ```
pub fn compute_length_stats(length_counts: &BTreeMap<usize, u64>) -> HashMap<String, f64> {
    //first get all the totals
    let (total_bases, total_seqs) = compute_total_counts(length_counts);
    let median_length = compute_median_length(length_counts, total_seqs);

    //now put the composite stats together
    let final_stats: HashMap<String, f64> = [
        ("total_bases".to_string(), total_bases as f64),
        ("total_sequences".to_string(), total_seqs as f64),
        ("mean_length".to_string(), (total_bases as f64) / (total_seqs as f64)),
        ("median_length".to_string(), median_length)
    ].iter().cloned().collect();
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
    fn test_full_all_same() {
        let seq_lens: BTreeMap<usize, u64> = [
            (10, 100)
        ].iter().cloned().collect();

        let expected_stats: HashMap<String, f64> = [
            ("total_bases".to_string(), 1000.0),
            ("total_sequences".to_string(), 100.0),
            ("mean_length".to_string(), 10.0),
            ("median_length".to_string(), 10.0)
        ].iter().cloned().collect();

        let actual_stats: HashMap<String, f64> = compute_length_stats(&seq_lens);
        assert_eq!(expected_stats, actual_stats);
    }
}