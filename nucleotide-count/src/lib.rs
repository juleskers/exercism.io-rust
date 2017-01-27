// Follow the complete history of this file, including all my painful learning
// experiences and conversations with the compiler at my github diary "Learning Rust":
// https://github.com/juleskers/exercism.io-rust/tree/master/nucleotide-count/src/lib.rs

use std::collections::HashMap;

pub fn count(nucleotide: char, sequence: &str)-> Result<usize, String> {
    if "ATGC".contains(nucleotide) {
        return Err(format!("Invalid nucleotide '{}', expected one of A, T, G or C", nucleotide));
    }
    Ok(0)
}

pub fn nucleotide_counts(sequence: &str) -> Result<HashMap<char, usize>, String> {
    unimplemented!();
}
