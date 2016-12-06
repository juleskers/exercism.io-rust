pub fn is_leap_year(year: i32) -> bool {
  // boring rule: divisible by 4?
  if year % 4 == 0 {
    // probably YES, oh wait, check centuries:
    if year % 100 == 0 {
      if year % 400 == 0 {
        // really big centuries are LEAP
        return true;
      } else {
        // it wasn't a 400-century: not leap!
        return false;
      }
    }
    // non-century 4th year
    return true;
  }
  // boring year!
  return false;
}
