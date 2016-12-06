pub fn raindrops(num: i32) -> String {
  // intermediate accumulator for drops
  let mut drops = String::new();

  // see which drops we need
  //   += seems to be new since rust 1.12.0: AddAssign
  if num % 3 == 0 { drops += "Pling"; }
  if num % 5 == 0 { drops += "Plang"; }
  if num % 7 == 0 { drops += "Plong"; }
  
  // Did we collect any drops?
  if drops.is_empty() {
    // no drops: return boring digits
    format!("{}", num)
  } else {
    // return our pretty drops! 
    drops
  }
}
