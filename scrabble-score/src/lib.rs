/// Calculates the scrabble-score of a provided word
///   Accented characters (ñ, é, etc) count for zero points.
pub fn score(word: &str) -> u16 {
  let mut score = 0;

  // buffer to store whatever value we added in the previous iteration
  //   We need this in case we stumble over a :double or :tripple marker for letters
  let mut prev_score = 0;

  // to keep iteration simple, we reduce the ':double' and ':triple' tokens to a single char
  let tokenised = word.to_lowercase().replace(":double", "2");

  // iterate over the characters in the word
  for the_char in tokenised.chars() {
    prev_score = match the_char {
      'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => 1,
      'd'|'g' => 2,
      'b'|'c'|'m'|'p' => 3,
      'f'|'h'|'v'|'w'|'y' => 4,
      'k' => 5,
      'j'|'x' => 8,
      'q'|'z' => 10,
      '2' => prev_score,   // 'keep' the previous value, so it is added again, doubling the previous char's value
      _   => 0, // 'funny' characters, such as accented ones, count for 0
    };

    score += prev_score;
  }

  score
}
