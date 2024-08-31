#[allow(dead_code)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f32>,
}

#[allow(dead_code)]
impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.data[row * self.cols + col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row * self.cols + col]
    }
}
