use geneticbreakout::matrix::matrix::Matrix;

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
