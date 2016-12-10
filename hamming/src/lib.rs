pub fn hamming_distance(l: &str, r: &str) -> Result<u16, ()> {
  let mut distance = 0;
  for (el, er) in l.chars().zip(r.chars()) {
    if el != er { distance += 1; }
  }
  
  return Ok(distance);
}
