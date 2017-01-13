pub fn sum_of_multiples(limit: u32, to_multiply: &[u32]) -> u32 {
  let to_multiply_max = to_multiply.iter().max().unwrap();

  // as u32 +1: we round up, because a to_multiply lower-than-max
  //  may stay below the limit while the max goes over the limit.
  //  to compensate, we filter in a later stage
  let repeat_limit = (limit/to_multiply_max) as u32 + 1; 

  let all_multiples: Vec<u32> = (1..repeat_limit+1).map(|r| {
    to_multiply.iter().map(|m| m*r)
  }).collect();

  match to_multiply.iter().filter(|n| n < &&limit ).next() {
    Some(v) => *v,
    None    =>  0,
  }
}
