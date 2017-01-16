pub fn score(word: &str) -> u16 {
  let mut score = 0;
  for the_char in word.to_lowercase().chars() {
    match the_char {
      'a'|'e'|'i'|'u'|'r'|'s'|'t'|'o' => score += 1,
      'f'|'y' => score += 4,
      'k' => score += 5,
      'q'|'z' => score += 10,
      _   => unimplemented!(),
    }
  }
  score
}
