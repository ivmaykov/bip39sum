use sha2::{Digest, Sha256};
use std::{env, error::Error};

fn bytes_to_mnemonic(bytes: &[u8], word_sep: &str) -> Result<String, bip39::Error> {
    let mnemonic = bip39::Mnemonic::from_entropy_in(bip39::Language::English, &bytes)?;
    let result_vec: Vec<&'static str> = mnemonic.word_iter().collect();
    Ok(result_vec.join(word_sep))
}

fn print_usage(program_name: &str) {
    println!("Usage: {} [FILE]...", program_name);
    println!(
        "Human-friendly file checksum program - prints file SHA256 hashes as BIP-39 mnemonics."
    );
    println!("For each input file, this program outputs a line in the format: \"BIP39 SHA256 FILE_NAME\", where:");
    println!("  BIP39 is the BIP39 mnemonic equivalent of the SHA256 hash, with underscore (_) word separators, and");
    println!("  SHA256 is a hex-encoded SHA256 hash of the file,");
    println!("  FILE_NAME is the input filename.");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_usage(&args[0]);
        return Ok(());
    }

    // If args are given, assume each arg is a file name.
    // For each file name, compute a sha256 of the file, convert it to a bip39 mnemonic,
    // and output the result, one per line.
    let mut iter = args.iter();
    iter.next(); // skip program name
    for file_name in iter {
        let mut hasher = Sha256::new();
        let mut file = std::fs::File::open(file_name)?;
        std::io::copy(&mut file, &mut hasher)?;
        let bytes = hasher.finalize();
        let hex_bytes = hex::encode(&bytes);
        println!(
            "{} {} {}",
            bytes_to_mnemonic(&bytes, "_")?,
            hex_bytes,
            file_name
        );
    }
    Ok(())
}
