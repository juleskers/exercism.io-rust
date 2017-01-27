// Follow the complete history of this file, including all my painful learning
// experiences and conversations with the compiler at my github diary "Learning Rust":
// https://github.com/juleskers/exercism.io-rust

// This obviously broken implementation still passes all tests.
// I tried to follow Test-driven Design principles to the point of obnoxiousness this time around.
// TDD states that:
//   A) you should never add code without a failing test-case (otherwise you're writing untested code, which will bite you later)
//   B) you should only add the minimal amount of code to make the test pass (otherwise you're writing untested code, see above)
//   C) (Not yet applied, will come) You should add tests that expose missing logic, one missing bit at a time
//   D) You can only refactor after you pass tests (adage: "Red, Green, Refactor")

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
