use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, to_multiply: &[u32]) -> u32 {
  if to_multiply.is_empty() {
    return 0;
  }

  let to_multiply_min = to_multiply.iter().min().unwrap();

  // as u32 +1: we round up, because a to_multiply lower-than-max
  //  may stay below the limit while the max goes over the limit.
  //  to compensate, we filter in a later stage
  let repeat_limit = (limit/to_multiply_min) as u32 + 1;

  let mut all_multiples: HashSet<u32> = HashSet::new();
  for r in 1..repeat_limit+1 {
    println!("repeat {}", r);
    for m in to_multiply {
      let candidate = r*m;
      println!("  candidate: {}", candidate);
      if candidate < limit {
        all_multiples.insert(candidate);
        println!("    added!");
      }
    }
  }

  all_multiples.iter().sum()
}
