pub fn score(word: &str) -> u16 {
  let mut score = 0;
  for the_char in word.to_lowercase().chars() {
    match the_char {
      'a'|'t' => score += 1,
      'f' => score += 4,
      _   => unimplemented!(),
    }
  }
  score
}
