use geneticbreakout::matrix::matrix::Matrix;
use geneticbreakout::matrix::ops;

#[test]
fn test_data() {
    let mut m = Matrix::new(2, 2);

    assert_eq!(m.get(0, 0), 0.0);
    assert_eq!(m.get(1, 1), 0.0);

    m.set(0, 0, 1.0);
    m.set(1, 1, 2.0);
    m.set(1, 0, 3.0);
    m.set(0, 1, 4.0);

    assert_eq!(m.get(0, 0), 1.0);
    assert_eq!(m.get(1, 1), 2.0);
    assert_eq!(m.get(1, 0), 3.0);
    assert_eq!(m.get(0, 1), 4.0);
}

#[test]
fn test_shape() {
    let m = Matrix::new(2, 3);
    assert_eq!(m.rows(), 2);
    assert_eq!(m.cols(), 3);
    assert_eq!(m.shape(), (2, 3));
}

#[test]
fn test_eq() {
    let mut m1 = Matrix::new(2, 3);
    let mut m2 = Matrix::new(2, 3);
    assert!(m1 == m1);
    assert!(m1 == m2);

    m1.set(0, 0, 1.0);
    assert!(m1 == m1);
    assert!(m1 != m2);

    m2.set(0, 0, 1.0);
    assert!(m1 == m2);

    let mut m1 = Matrix::new(2, 3);
    let mut m2 = Matrix::new(3, 2);
    assert!(m1 != m2);

    m1.set(0, 0, 1.0);
    m2.set(0, 0, 1.0);
    assert!(m1 != m2);
}

#[test]
fn test_matmul() {
    let mut a = Matrix::new(2, 3);
    for i in 0..2 {
        for j in 0..3 {
            a.set(i, j, (i * 3 + j) as f32);
        }
    }
}
