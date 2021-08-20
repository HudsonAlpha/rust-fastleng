
use std::collections::HashMap;

/// TODO
pub fn compute_total_counts(length_metrics: &HashMap<usize, u64>) -> (u64, u64) {
    let mut total_bases: u64 = 0;
    let mut total_seqs: u64 = 0;
    for (length, count) in length_metrics.iter() {
        total_bases += (*length as u64) * count;
        total_seqs += count;
    }
    (total_bases, total_seqs)
}

/// TODO
pub fn compute_length_stats(length_metrics: &HashMap<usize, u64>) -> HashMap<String, f64> {
    //first get all the totals
    let (total_bases, total_seqs) = compute_total_counts(length_metrics);

    //now put the composite stats together
    let final_stats: HashMap<String, f64> = [
        ("total_bases".to_string(), total_bases as f64),
        ("total_sequences".to_string(), total_seqs as f64)
    ].iter().cloned().collect();
    final_stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_total_counts() {
        let seq_lens: HashMap<usize, u64> = [
            (10, 100)
        ].iter().cloned().collect();

        let expected_total_bases: u64 = 1000;
        let expected_total_seqs: u64 = 100;

        let computed_results = compute_total_counts(&seq_lens);
        assert_eq!((expected_total_bases, expected_total_seqs), computed_results);
    }

    #[test]
    fn test_full_all_same() {
        let seq_lens: HashMap<usize, u64> = [
            (10, 100)
        ].iter().cloned().collect();

        let expected_stats: HashMap<String, f64> = [
            ("total_bases".to_string(), 1000.0),
            ("total_sequences".to_string(), 100.0)
            //("mean", 10),
            //("median", 10)
        ].iter().cloned().collect();

        let actual_stats: HashMap<String, f64> = compute_length_stats(&seq_lens);
        assert_eq!(expected_stats, actual_stats);
    }
}