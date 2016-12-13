pub struct PascalsTriangle {
  row_buffer: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        // prepare the Pascal triangle
        // We know how many rows we'll have, save some Vec re-allocations with that knowledge
        let mut rb = Vec::with_capacity(row_count as usize);

        // Fill the rows, one by one
        for i in 1..row_count+1 {
            rb.push(vec![1])
        }

        // wrap our new triangle in a PT struct for returning
        PascalsTriangle {
            row_buffer: rb
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.row_buffer.clone() // .clone() so that callers can't mess with our internal buffers
    }
}
