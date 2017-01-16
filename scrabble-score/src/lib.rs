pub fn score(word: &str) -> u16 {
  let mut score = 0;
  for the_char in word.to_lowercase().chars() {
    match the_char {
      'a'|'e'|'r'|'s'|'t'|'o' => score += 1,
      'f' => score += 4,
      'z' => score += 10,
      _   => unimplemented!(),
    }
  }
  score
}
