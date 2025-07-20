use noob_linalg::Matrix;
fn main() {
    let matrix = Matrix::from_file("src/3x3.txt", 3, 3);
    println!("{:?}: {}", matrix, matrix.det());
}
