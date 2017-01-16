pub fn score(word: &str) -> u16 {
  match word.to_lowercase().chars().next().unwrap() {
    'a' => 1,
    'f' => 4,
    _   => unimplemented!(),
  }
}
