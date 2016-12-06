/// Capitalises the first letter of the supplied string.
fn capitalise(s: String) -> String {
  // thank you Stackoverflow:
  // http://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
  let mut chars = s.chars();
  match chars.next() {
    None => String::new(),
    Some(c) => c.to_uppercase().collect::<String>() + chars.as_str()
  }
}

/// turns a positive integer into a 'spoken' bottle count.
fn bottle_word(bottle_count: usize) -> String {
  match bottle_count {
    0 => String::from("no more bottles"),
    1 => String::from("1 bottle"),
    _ => format!("{} bottles", bottle_count),
  }
}

/// given a starting bottle count, returns the take-down part of the verse.
fn takedown_fragment(bottle_count: usize) -> String {
  match bottle_count {
    0 => String::from("Go to the store and buy some more"),
    _ => format!("Take {} down and pass it around", match bottle_count { 1 => "it", _ => "one" })
  }
}

/// Given a 'before' bottlecount, returns the text form of how many bottles we have afterwards.
fn bottles_after(bottle_count: usize) -> String {
  bottle_word(match bottle_count {
    0 => 99,
    _ => bottle_count - 1,
  })
}

/// Returns the complete verse for any (positive) number of bottles.
pub fn verse(bottle_count: usize) -> String {
  // complete verse-solution; after refactoring
  format!(
       "{bottles_capital} of beer on the wall, {bottles} of beer.\n{takedown}, {post_bottles} of beer on the wall.\n",
       bottles_capital = capitalise(bottle_word(bottle_count)), 
       bottles = bottle_word(bottle_count), 
       takedown = takedown_fragment(bottle_count),
       post_bottles = bottles_after(bottle_count),
  )
}

/// Returns the complete song, starting at start_bottles, and stopping after taking down the last_bottle_taken_down.
pub fn sing(start_bottles: usize, last_bottle_taken_down: usize) -> String {
  (last_bottle_taken_down..start_bottles+1) // the range, must be in ascending order because step_by(-1) isn't stable yet
  .rev()                                    // switch to descending, beer-take-down order
  .map(|b| verse(b))                        // get the individual verses for each bottle_count
  .collect::<Vec<_>>()                      // compile verses into a list. (with minimal type-hint to make it compile)
  .join("\n")                               // and finally, join them in a single string, with nice linebreak between verses.
}
