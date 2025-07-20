use std::ops::{Index, IndexMut};

use rand::seq::index;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix{
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix{
    pub fn new(rows: usize, cols: usize) -> Self{
        Self{cols,rows, data: vec![0.0; rows * cols]}
    }

    pub fn identity(size: usize) -> Self{
        let mut data: Vec<f64> = vec![0.0; size * size];
        for i in 0..size{
            data[i * size + i] = 1.0;
        }
        Self{cols: size, rows: size, data}
    }
}

impl Index<usize> for Matrix{
    type Output = [f64];
    fn index(&self, index: usize) -> &Self::Output{
        let start = index * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
    }
}

impl IndexMut<usize> for Matrix{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output{
        let start = index * self.cols;
        let end = start + self.cols;
        &mut self.data[start..end]
    }
}

