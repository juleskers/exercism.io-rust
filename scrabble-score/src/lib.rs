pub fn score(word: &str) -> u16 {
  let mut score = 0;
  for the_char in word.to_lowercase().chars() {
    score += match the_char {
      'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => 1,
      'd'|'g' => 2,
      'b'|'c'|'m'|'p' => 3,
      'f'|'h'|'v'|'w'|'y' => 4,
      'k' => 5,
      'j'|'x' => 8,
      'q'|'z' => 10,
      _   => 0,
    }
  }
  score
}
