// Follow the complete history of this file, including all my painful learning
// experiences and conversations with the compiler at my github diary "Learning Rust":
// https://github.com/juleskers/exercism.io-rust

pub fn is_pangram(candidate: &str) -> bool {
    if candidate.is_empty() 
            || !candidate.contains('e')
            || !candidate.contains('p')
            || !candidate.contains('x')
            || !candidate.to_lowercase().contains('z') {
        return false;
    } else {
        true
    }
    
    
}
