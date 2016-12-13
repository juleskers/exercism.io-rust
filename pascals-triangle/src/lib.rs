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
            // prepare the current row
            let mut new_row = Vec::with_capacity(i as usize); // It's a triangle: each row is as long as it is down
            
            // fill the current row
            for j in 1..i+1 {
                match j {
                   // first and last item of a row are always '1', because they're on the edge, with an "n/a" and a "1" above them
                   1         => new_row.push(1),
                   _ if i==row_count => new_row.push(1),
                   // all other rows are sum of previous elements above them
                   _ => unimplemented!(),
                }
            }
            
            rb.push(new_row)
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
