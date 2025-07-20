use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            cols,
            rows,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut data: Vec<f64> = vec![0.0; size * size];
        for i in 0..size {
            data[i * size + i] = 1.0;
        }
        Self {
            cols: size,
            rows: size,
            data,
        }
    }
    fn from_vec(data: Vec<f64>, rows: usize, cols: usize) -> Self {
        Self { rows, cols, data }
    }
    pub fn from_file(path: &str, rows: usize, cols: usize) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut data: Vec<f64> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let iter = line.split_whitespace();
            for num in iter {
                data.push(num.parse().unwrap());
            }
        }
        Self { rows, cols, data }
    }
    // Do this recursively for now
    pub fn det(&self) -> f64 {
        if self.rows != self.cols {
            panic!("Matrix is not square");
        }
        let mut det = 0.0;
        if self.rows == 1 {
            det += self.data[0];
        } else if self.rows == 2 {
            det += self.data[0] * self.data[3] - self.data[1] * self.data[2];
        } else {
            for i in 0..self.rows {
                let mut vec: Vec<f64> = Vec::new();

                for j in 1..self.rows {
                    for k in 0..self.cols {
                        if k == i {
                            continue;
                        }
                        vec.push(self[j][k]);
                    }
                }

                let coeff = if i % 2 == 0 { 1.0 } else { -1.0 };
                det +=
                    coeff * self[0][i] * Matrix::from_vec(vec, self.rows - 1, self.cols - 1).det();
            }
        }
        det
    }
}

impl Index<usize> for Matrix {
    type Output = [f64];
    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &mut self.data[start..end]
    }
}
