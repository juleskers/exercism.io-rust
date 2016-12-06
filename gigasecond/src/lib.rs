extern crate chrono;
use chrono::*;

pub fn after(birthday: chrono::DateTime<UTC>) -> chrono::DateTime<UTC> {
  let gigasecond = Duration::seconds(1_000_000_000);
  
  birthday.checked_add(gigasecond).unwrap()
}
