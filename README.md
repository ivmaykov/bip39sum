# Bip39sum - human-friendly file checksum tool
Bip39sum is a human-friendly tool for calculating file checksums. It uses the SHA256 cryptographic hash function and Bitcoin's BIP39 mnemonic algorithm to output human-friendly checksums of files. Such checksums are much easier to verify by hand than the output of `sha256sum`, which is a hex-encoded 64-character hex string. Bip39sum also outputs the sha256 sums so the checksum computed by the tool can be compared to the output of `sha256sum` if desired.

Currently the tool is only available in source code form and has to be compiled by users. It is written in [Rust](https://www.rust-lang.org/) and depends on the [bip39](https://crates.io/crates/bip39), [hex](https://crates.io/crates/hex), and [sha2](https://crates.io/crates/sha2) crates.

## Building the code
Requirements: git, rust.
```
git clone https://github.com/ivmaykov/bip39sum
cd bip39sum
cargo build
```

## Usage
```
$ cargo run src/main.rs
rule_oak_tag_invest_script_smile_all_engine_magic_give_script_error_quick_hungry_expose_follow_immense_fat_patient_over_isolate_tornado_curtain_warrior bd32fb74bafc1b99419a5385cc4f06a66afade942ad5718a6e844ef769cacd8f src/main.rs
```

## License
This code is licensed under the MIT license.
