// I was wondering why this didn't cause an overflow-panic, so here are some numbers from my analysis:
//
// u64::max_value()     18_446_744_073_709_551_615  // built-in constant, the max you can stuff in 64 unsigned bits
// grains::square(64):   9_223_372_036_854_775_808  // 2^(64-1); 63 because square 1 starts with 1 grain (=2^0), not 2 grains (=2^1) (AHA!)
// grains::total:       18_446_744_073_709_551_615  // 2^0 + 2^1 + 2^2 ... 2^63 = (2^64)-1 = u64::max_value()
//
// Conclusion: we are saved from overflow (by the skin of our teeth!) by the problem-statement.
//  It is an inherent property of all binary numbers that the sum of all previous bits is worth 1 less than the current bit
//  4-1 = 2+1, 8-1 = 4+2+1, 16-1 = 8+4+2+1, etc
// Because our chessboard starts it's power-series with 2^0 = 2^(square_number - 1), our last square(64) is 2^(64-1) = 2^63.
// Then the binary series rule we just 'discovered' ensures the sum of 2^0..2^63 = (2^64)-1, conveniently our max-value

pub fn square(s: u32) -> u64 {
    match s {
        1...64 => 2u64.pow(s - 1),
        _     => panic!("Square must be between 1 and 64"),
    }
    
}

pub fn total() -> u64 {
    (1..65).map(|s| square(s)).sum()
}
