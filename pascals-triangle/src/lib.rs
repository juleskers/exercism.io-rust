pub struct PascalsTriangle {
  row_buffer: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_buffer: Vec::new()
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.row_buffer.clone() // .clone() so that callers can't mess with our internal buffers
    }
}
