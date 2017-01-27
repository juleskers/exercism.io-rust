// Follow the complete history of this file, including all my painful learning
// experiences and conversations with the compiler at my github diary "Learning Rust":
// https://github.com/juleskers/exercism.io-rust/tree/master/nucleotide-count/src/lib.rs

extern crate boolinator;

use std::collections::HashMap;
use boolinator::Boolinator;

pub fn count(nucleotide: char, sequence: &str)-> Result<usize, String> {
    let mut count = 0;

    validate(nucleotide).or (
        Err(format!("Invalid nucleotide '{}', expected one of A, T, G or C", nucleotide))
    )?;

    nucleotide_counts(sequence).and_then(|nc|
        Ok(nc.get(&nucleotide)
          .expect("Programmer Error: valid nucleotide not present in count result!"))
    )
}

fn validate(nuc: char) -> Result<(), ()> {
    "ATGC".contains(nuc).as_result((),())
}

pub fn nucleotide_counts(sequence: &str) -> Result<HashMap<char, usize>, String> {
    let mut counts = HashMap::with_capacity(4);
    counts.insert('A', 0);
    counts.insert('T', 0);
    counts.insert('G', 0);
    counts.insert('C', 0);

    for s in sequence.chars() {
        validate(s).is_ok().ok_or(format!("Invalid nucleotide in sequence: '{}', expected one of A, T, G or C", s))?;

        *(counts.get_mut(&s)
        .expect("Programmer error: valid nucleotide not present in counts-map"))
        += 1;
    }

    Ok(counts)
}
