# Bip39sum - human-friendly file checksum tool
Bip39sum is a human-friendly tool for calculating file checksums. It uses the SHA256 cryptographic hash function represented as a Bitcoin BIP39 mnemonic to output human-friendly checksums of files. Such checksums are much easier to verify by hand than the output of `sha256sum` - 24 distinctive words vs. 64 hexadecimal characters. Bip39sum also outputs the sha256 sums so the checksum computed by the tool can be compared to the output of `sha256sum` if desired.

Currently the tool is only available in source code form and has to be compiled by users. It is written in [Rust](https://www.rust-lang.org/) and depends on the [bip39](https://crates.io/crates/bip39) and [sha2](https://crates.io/crates/sha2) crates.

## Building the code
Requirements: git, rust.
```
git clone https://github.com/ivmaykov/bip39sum
cd bip39sum
cargo build
```

## Usage
The tool can be run with `cargo run`:
```
$ cargo run ./src/main.rs
human_exclude_resist_choose_arrest_horn_online_piano_gauge_hobby_math_position_cushion_fault_ankle_faculty_naive_arm_blouse_weasel_noble_canoe_twice_focus 6ec9dedd9420c6db26bd20608d8a24543366a7c24a8d92a17460fc495a433ad2 ./src/main.rs
```
The binary can also be invoked directly:
```
$ ./target/debug/bip39sum ./src/main.rs
human_exclude_resist_choose_arrest_horn_online_piano_gauge_hobby_math_position_cushion_fault_ankle_faculty_naive_arm_blouse_weasel_noble_canoe_twice_focus 6ec9dedd9420c6db26bd20608d8a24543366a7c24a8d92a17460fc495a433ad2 ./src/main.rs
```

## License
This code is licensed under the MIT license.
