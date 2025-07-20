
use noob_linalg::Matrix;
fn main() {
    let mut matrix = Matrix::new(3, 3);
    let matrix2: Matrix = Matrix::identity(3);
    matrix[2][2] = 10.0;
    println!("{:?} {:?}", matrix, matrix2);
}
