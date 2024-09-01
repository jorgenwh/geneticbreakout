use crate::matrix::matrix::Matrix;

pub fn matmul(a: &Matrix, b: &Matrix) -> Matrix {
    assert_eq!(a.cols(), b.rows());

    let mut result = Matrix::new(a.rows(), b.cols());

    for i in 0..a.rows() {
        for j in 0..b.cols() {
            let mut sum = 0.0;
            for k in 0..a.cols() {
                sum += a.get(i, k) * b.get(k, j);
            }
            result.set(i, j, sum);
        }
    }

    result
}

