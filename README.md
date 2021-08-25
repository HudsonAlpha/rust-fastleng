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
The following command will invoke `fastleng` on a given FASTQ file:
```
fastleng {data.fq.gz}
```

### Output format

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