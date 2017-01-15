pub fn sum_of_multiples(limit: u32, to_multiply: &[u32]) -> u32 {
  let to_multiply_max = to_multiply.iter().max().unwrap();

  // as u32 +1: we round up, because a to_multiply lower-than-max
  //  may stay below the limit while the max goes over the limit.
  //  to compensate, we filter in a later stage
  let repeat_limit = (limit/to_multiply_max) as u32 + 1; 

  let mut sum: u32 = 0;
  for r in (1..repeat_limit+1) {
    println!("repeat {}", r);
    for m in to_multiply {
      let candidate = r*m;
      println!("  candidate: {}", candidate);
      if candidate < limit {
        sum += candidate;
        println!("    added, sum now {}!", sum);
      }
    }
  }

  sum
}
