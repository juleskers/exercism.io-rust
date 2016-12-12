pub struct PascalsTriangle {
  row_buffer: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let pt = PascalsTriangle {
            row_buffer: Vec::with_capacity(row_count as usize)
        };
        
        for i in 1..row_count {
          pt.row_buffer.push(vec![1])
        }
        
        pt
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.row_buffer.clone() // .clone() so that callers can't mess with our internal buffers
    }
}
