// Follow the complete history of this file, including all my painful learning
// experiences and conversations with the compiler at my github diary "Learning Rust":
// https://github.com/juleskers/exercism.io-rust

pub fn is_pangram(candidate: &str) -> bool {
    // fail fast on simple cases
    if candidate.is_empty() {
        return false;
    }

    // Stop being obnoxious, and have a full implementation
    //  test-suite also expanded, see github:
    //  https://github.com/juleskers/exercism.io-rust/commit/47a1e75f9596cbd8a2d7acfd38f6b4b39bd9b74f

    let candidate_normalised = candidate.to_lowercase();
    if         !candidate_normalised.contains('a')
            || !candidate_normalised.contains('b')
            || !candidate_normalised.contains('c')
            || !candidate_normalised.contains('d')
            || !candidate_normalised.contains('e')
            || !candidate_normalised.contains('f')
            || !candidate_normalised.contains('g')
            || !candidate_normalised.contains('h')
            || !candidate_normalised.contains('i')
            || !candidate_normalised.contains('j')
            || !candidate_normalised.contains('k')
            || !candidate_normalised.contains('l')
            || !candidate_normalised.contains('m')
            || !candidate_normalised.contains('n')
            || !candidate_normalised.contains('o')
            || !candidate_normalised.contains('p')
            || !candidate_normalised.contains('q')
            || !candidate_normalised.contains('r')
            || !candidate_normalised.contains('s')
            || !candidate_normalised.contains('t')
            || !candidate_normalised.contains('u')
            || !candidate_normalised.contains('v')
            || !candidate_normalised.contains('w')
            || !candidate_normalised.contains('x')
            || !candidate_normalised.contains('y')
            || !candidate_normalised.contains('z')
    {
        false
    } else {
        true
    }
}
