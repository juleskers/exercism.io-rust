pub fn score(word: &str) -> u16 {
  let mut score = 0;
  for the_char in word.to_lowercase().chars() {
    match the_char {
      'a'|'e'|'i'|'o'|'u'|'n'|'r'|'s'|'t' => score += 1,
      'b'|'p' => score += 3,
      'f'|'h'|'y' => score += 4,
      'k' => score += 5,
      'x' => score += 8,
      'q'|'z' => score += 10,
      _   => unimplemented!(),
    }
  }
  score
}
