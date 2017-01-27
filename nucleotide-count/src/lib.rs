// Follow the complete history of this file, including all my painful learning
// experiences and conversations with the compiler at my github diary "Learning Rust":
// https://github.com/juleskers/exercism.io-rust/tree/master/nucleotide-count/src/lib.rs

use std::collections::HashMap;

pub fn count(nucleotide: char, sequence: &str)-> Result<usize, String> {

    valid_nucleotide(nucleotide).or (
        Err(format!("Invalid nucleotide '{}', expected one of A, T, G or C", nucleotide))
    )?;

    if sequence.chars().any(|s| valid_nucleotide(s).is_err()) {
        return Err(format!("Invalid nucleotide in sequence: '{}', expected one of A, T, G or C", nucleotide));
    }

    Ok(0)
}

fn valid_nucleotide(nuc: char) -> Result<(), ()> {
    if "ATGC".contains(nuc) {
        Ok(())
    } else {
        Err(())
    }
}

pub fn nucleotide_counts(sequence: &str) -> Result<HashMap<char, usize>, String> {
    unimplemented!();
}
