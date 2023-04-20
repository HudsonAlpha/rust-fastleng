[![Crates.io](https://img.shields.io/crates/d/fastleng.svg)](https://crates.io/crates/fastleng)
[![Crates.io](https://img.shields.io/crates/v/fastleng.svg)](https://crates.io/crates/fastleng)
[![Crates.io](https://img.shields.io/crates/l/fastleng.svg)](https://crates.io/crates/fastleng)
[![Build status](https://github.com/HudsonAlpha/rust-fastleng/actions/workflows/quickstart-ci.yml/badge.svg)](https://github.com/HudsonAlpha/rust-fastleng/actions)

# rust-fastleng
`fastleng` is a tool created specifically for gathering sequence length information from a FASTQ, FASTA, or unaligned BAM file.

### Why another FASTX stat tool?
While there are numerous tools that will generate summary statistics for FASTX files, I was not able to find one that computed all the desired length metrics for _both_ FASTQ and FASTA.
[pyfastx](https://pyfastx.readthedocs.io/en/latest/) was the closest, but it seems to limit certain statistics (e.g. N50) to only one file type.

In constrast, aside from the initial parsing, `fastleng` is agnostic to the file type.
However, it is (currently) focused only on generating metrics derived from the sequence lengths.
For more comprehensive metrics, it may be better to use tools like [pyfastx](https://pyfastx.readthedocs.io/en/latest/) or [fastp](https://github.com/OpenGene/fastp).

## Installation
All installation options assume you have installed [Rust](https://www.rust-lang.org) along with the `cargo` crate manager for Rust.

### From Cargo
```bash
cargo install fastleng
fastleng -h
```

### From GitHub
```bash 
git clone https://github.com/HudsonAlpha/rust-fastleng.git
cd rust-fastleng
#testing optional
cargo test --release
cargo build --release
./target/release/fastleng -h
#for local install
cargo install --path .
```

## Usage
### Typical Usage
The following command will invoke `fastleng` on a given FASTQ file and redirect the results from stdout into a JSON file:
```
fastleng {data.fq.gz} > {output.json}
```

### Example output
```
{
  "total_bases": 21750112406,
  "total_sequences": 1305936,
  "mean_length": 16654.807284583625,
  "median_length": 16600.0,
  "n10": 18849,
  "n25": 17833,
  "n50": 16739,
  "n75": 15842,
  "n90": 15209
}
```
1. `total_bases` - the total number of basepairs across all sequences in the input file
2. `total_sequences` - the total number of sequences (i.e. strings) contained in the input file
3. `mean_length` - the average length of the counted sequences
4. `median_length` - the median length of the counted sequences
5. `n10`, `n25`, `n50`, `n75`, `n90` - the [N-score](https://en.wikipedia.org/wiki/N50,_L50,_and_related_statistics) of the sequences for 10, 25, 50, 75, and 90 respectively; these should be monotonically decreasing, respectively

### Options to consider
1. `-h` - see full list of options and exit
2. `-l`, `--length-json` - enables the saving of the raw length counts to a specified JSON file
3. `-o`, `--out-json` - enabled used to specify the filename to write the length statistics to (default: stdout)

## TODO List
1. Create an option for other N-score values (or maybe all integer N-score values)
2. If you have other length-based statistics, feel free to open a feature request on GitHub.

## Performance notes
We have not performed formal benchmarking.
Anecdotally, the vast majority of the run-time is spent loading the FASTX file, so the program is very I/O bound currently.

## Reference
Fastleng does not currently have a pre-print or paper associated with it.

## License
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
