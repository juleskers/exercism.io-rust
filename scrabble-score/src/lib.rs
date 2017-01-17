pub fn score(word: &str) -> u16 {
  let mut score = 0;
  for the_char in word.to_lowercase().chars() {
    score += match the_char {
      'a'|'e'|'i'|'o'|'u'|'n'|'r'|'s'|'t' => 1,
      'b'|'p' => 3,
      'f'|'h'|'y' => 4,
      'k' => 5,
      'x' => 8,
      'q'|'z' => 10,
      _   => unimplemented!(),
    }
  }
  score
}
