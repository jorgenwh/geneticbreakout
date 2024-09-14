use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f32>,
}

#[allow(dead_code)]
impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        if rows <= 0 || cols <= 0 {
            panic!("Matrix must have at least one row and one column");
        }
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        if rows <= 0 || cols <= 0 {
            panic!("Matrix must have at least one row and one column");
        }
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn ones(rows: usize, cols: usize) -> Matrix {
        if rows <= 0 || cols <= 0 {
            panic!("Matrix must have at least one row and one column");
        }
        Matrix {
            rows,
            cols,
            data: vec![1.0; rows * cols],
        }
    }

    pub fn arange(rows: usize, cols: usize) -> Matrix {
        if rows <= 0 || cols <= 0 {
            panic!("Matrix must have at least one row and one column");
        }
        let mut data = Vec::with_capacity(rows * cols);
        for i in 0..rows * cols {
            data.push(i as f32);
        }
        Matrix { rows, cols, data }
    }

    pub fn from_data(rows: usize, cols: usize, data: Vec<f32>) -> Matrix {
        if rows <= 0 || cols <= 0 {
            panic!("Matrix must have at least one row and one column");
        }
        if data.len() != rows * cols {
            panic!("Data length does not match matrix shape");
        }
        Matrix { rows, cols, data }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn reshape(&mut self, rows: usize, cols: usize) {
        if rows <= 0 || cols <= 0 {
            panic!("Matrix must have at least one row and one column");
        }
        if rows * cols != self.rows * self.cols {
            panic!("New shape must have the same number of elements");
        }
        self.rows = rows;
        self.cols = cols;
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.data[row * self.cols + col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row * self.cols + col]
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:.2} ", self.get(i, j))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
