// Follow the complete history of this file, including all my painful learning
// experiences and conversations with the compiler at my github diary "Learning Rust":
// https://github.com/juleskers/exercism.io-rust/tree/master/nucleotide-count/src/lib.rs

extern crate boolinator;

use std::collections::HashMap;
use boolinator::Boolinator;

/// Counts the occurances of a given Nucleotide (A,T,G,C) in a sequence
pub fn count(nucleotide: char, sequence: &str)-> Result<usize, String> {
    validate(nucleotide).or (
        Err(format!("Invalid nucleotide '{}', expected one of A, T, G or C", nucleotide))
    )?;

    nucleotide_counts(sequence).and_then(|nc|
        Ok(*nc.get(&nucleotide)
          .expect("Programmer Error: valid nucleotide not present in count result!"))
    )
}

/// Checks if a character is a valid nucleotide (A,T,G,C)
fn validate(nuc: char) -> Result<(), ()> {
    "ATGC".contains(nuc).as_result((),())
}

/// Returns the counts of all nucleotides (A,T,G,C) in the sequence
pub fn nucleotide_counts(sequence: &str) -> Result<HashMap<char, usize>, String> {
    // prepare our counts map
    let mut counts = HashMap::with_capacity(4);
    counts.insert('A', 0);
    counts.insert('T', 0);
    counts.insert('G', 0);
    counts.insert('C', 0);

    // Check our entire sequence...
    for s in sequence.chars() {
        // Check if the current position is OK, or bail out (with the '?' syntax)
        validate(s).or(Err(
            format!("Invalid nucleotide in sequence: '{}', expected one of A, T, G or C", s)
        ))?;

        // increase the correct count
        *(counts.get_mut(&s)
        // expect should never trigger, since we validated the sequence char, and we hardcoded the four map-entries
        .expect("Programmer error: valid nucleotide not present in counts-map"))
        += 1;
    }

    Ok(counts)
}
