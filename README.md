[![Crates.io](https://img.shields.io/crates/d/fastleng.svg)](https://crates.io/crates/fastleng)
[![Crates.io](https://img.shields.io/crates/v/fastleng.svg)](https://crates.io/crates/fastleng)
[![Crates.io](https://img.shields.io/crates/l/fastleng.svg)](https://crates.io/crates/fastleng)
[![Build status](https://github.com/HudsonAlpha/rust-fastleng/actions/workflows/quickstart-ci.yml/badge.svg)](https://github.com/HudsonAlpha/rust-fastleng/actions)

# rust-fastleng
Fastleng is a tool create specifically for gathering sequence length information from a FASTQ or FASTA file.

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
  "total_bases": 1358218298,
  "total_sequences": 100000,
  "mean_length": 13582.18298,
  "median_length": 13664.0,
  "n50": 13775,
  "n75": 13027,
  "n90": 12543
}
```
1. `total_bases` - the total number of basepairs across all sequences in the input file
2. `total_sequences` - the total number of sequences (i.e. strings) contained in the input file
3. `mean_length` - the average length of the counted sequences
4. `median_length` - the median length of the counted sequences
5. `n50`, `n75`, `n90` - the [N-score](https://en.wikipedia.org/wiki/N50,_L50,_and_related_statistics) of the sequences for 50, 75, and 90 respectively 

### Options to consider
1. `-h` - see full list of options and exit

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