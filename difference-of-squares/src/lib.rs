pub fn square_of_sum(num: usize) -> usize {
  // type-hints for the win!
  //   "::<usize>" tells the compiler what comes out of our range.
  //   since the range can be converted to basically any numeric type, we need to help
  (1..num+1).sum::<usize>().pow(2)
}

pub fn sum_of_squares(num: usize) -> usize {
  (1..num+1).map(|x| x.pow(2)).sum()
}

pub fn difference(num: usize) -> usize {
  square_of_sum(num) - sum_of_squares(num)
}
