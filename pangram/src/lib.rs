// Follow the complete history of this file, including all my painful learning
// experiences and conversations with the compiler at my github diary "Learning Rust":
// https://github.com/juleskers/exercism.io-rust

pub fn is_pangram(candidate: &str) -> bool {
    let candidate_normalised = candidate.to_lowercase();

    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    alphabet.chars().all(|letter| candidate_normalised.contains(letter))
}
