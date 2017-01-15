/// See the full history of this file in my public github repo:
/// https://github.com/juleskers/exercism.io-rust/blob/master/sum-of-multiples/src/lib.rs

use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, to_multiply: &[u32]) -> u32 {
  // sanity test / fail-fast: We need SOMETHING to multiply
  if to_multiply.is_empty() {
    return 0;
  }

  // Find our iteration limit:
  //  The highest multiplication-factor below the limit is reached for the smallest number to_multiply.
  //  as u32 + 1: we round up to make sure we don't accidentally miss a multiple on the edge.
  let to_multiply_min = to_multiply.iter().min().unwrap();
  let repeat_limit = (limit/to_multiply_min) as u32 + 1;

  let mut all_multiples: HashSet<u32> = HashSet::new();
  for r in 1..repeat_limit+1 {
    all_multiples.extend(to_multiply.iter().map(|m| r*m));
  }
  let filtered_multiples = all_multiples.into_iter().filter(|e| e < &limit);

  filtered_multiples.into_iter().sum()
}
