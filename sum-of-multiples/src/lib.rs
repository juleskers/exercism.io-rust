pub fn sum_of_multiples(limit: u32, to_multiply: &[u32]) -> u32 {
  match to_multiply.iter().filter(|n| n < &&limit ).next() {
    Some(v) => *v,
    None    =>  0,
  }
}
