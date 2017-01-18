/// Calculates the scrabble-score of a provided word
///   Accented characters (ñ, é, etc) count for zero points.
pub fn score(word: &str) -> u16 {
  let mut score = 0;

  // iterate over the characters in the word
  for the_char in word.to_lowercase().chars() {
    score += match the_char {
      'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => 1,
      'd'|'g' => 2,
      'b'|'c'|'m'|'p' => 3,
      'f'|'h'|'v'|'w'|'y' => 4,
      'k' => 5,
      'j'|'x' => 8,
      'q'|'z' => 10,
      _   => 0, // 'funny' characters, such as accented ones, count for 0
    }
  }

  score
}
