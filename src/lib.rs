/*!
# Fastleng
This library provides access to the fastx loading and statistics functions necessary to gather sequence length statistics in `fastleng`.

## Example
```rust
use std::collections::BTreeMap;
use fastleng::fastx_loader::gather_fastx_stats;
use fastleng::length_stats::{compute_length_stats, LengthStats};

//load the fastx file lengths
let filename = "./test_data/long_strings.fa";
let length_counts: BTreeMap<usize, u64> = gather_fastx_stats(&filename).unwrap();

//compute the stats
let length_metrics: LengthStats = compute_length_stats(&length_counts);
```
*/

/// Contains the logic for loading length information from a fastx file
pub mod fastx_loader;
/// Contains the logic for calculating the summary statistics from the counts
pub mod length_stats;