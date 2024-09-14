use geneticbreakout::matrix::matrix::Matrix;
use geneticbreakout::matrix::ops;

#[test]
fn test_xet() {
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
fn test_reshape() {
    let mut m = Matrix::new(2, 3);
    for i in 0..2 {
        for j in 0..3 {
            m.set(i, j, (i * 3 + j) as f32);
        }

    }
    assert_eq!(m.rows(), 2);
    assert_eq!(m.cols(), 3);
    assert_eq!(m.shape(), (2, 3));

    m.reshape(3, 2);
    assert_eq!(m.rows(), 3);
    assert_eq!(m.cols(), 2);
    assert_eq!(m.shape(), (3, 2));
    for i in 0..3 {
        for j in 0..2 {
            assert_eq!(m.get(i, j), (i * 2 + j) as f32);
        }
    }
}

#[test]
fn test_new() {
    let m = Matrix::new(2, 3);
    assert_eq!(m.shape(), (2, 3));
    assert_eq!(m.rows(), 2);
    assert_eq!(m.cols(), 3);
    for i in 0..2 {
        for j in 0..3 {
            assert_eq!(m.get(i, j), 0.0);
        }
    }
}

#[test]
fn test_zeros() {
    let m = Matrix::zeros(4, 2);
    assert_eq!(m.shape(), (4, 2));
    assert_eq!(m.rows(), 4);
    assert_eq!(m.cols(), 2);
    for i in 0..4 {
        for j in 0..2 {
            assert_eq!(m.get(i, j), 0.0);
        }
    }
}

#[test]
fn test_ones() {
    let m = Matrix::ones(4, 2);
    assert_eq!(m.shape(), (4, 2));
    assert_eq!(m.rows(), 4);
    assert_eq!(m.cols(), 2);
    for i in 0..4 {
        for j in 0..2 {
            assert_eq!(m.get(i, j), 1.0);
        }
    }
}

#[test]
fn test_arange() {
    let m = Matrix::arange(1, 10);
    assert_eq!(m.shape(), (1, 10));
    assert_eq!(m.rows(), 1);
    assert_eq!(m.cols(), 10);
    for i in 0..10 {
        assert_eq!(m.get(0, i), i as f32);
    }

    let m = Matrix::arange(4, 6);
    assert_eq!(m.shape(), (4, 6));
    assert_eq!(m.rows(), 4);
    assert_eq!(m.cols(), 6);
    for i in 0..4 {
        for j in 0..6 {
            assert_eq!(m.get(i, j), (i * 6 + j) as f32);
        }
    }
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
    let mut b = Matrix::new(3, 2);
    for i in 0..2 {
        for j in 0..3 {
            a.set(i, j, (i * 3 + j) as f32);
            b.set(j, i, (i * 3 + j) as f32);
        }
    }

    let mut c_expected = Matrix::new(2, 2);
    c_expected.set(0, 0, 5.0);
    c_expected.set(0, 1, 14.0);
    c_expected.set(1, 0, 14.0);
    c_expected.set(1, 1, 50.0);
    let c = ops::matmul(&a, &b);

    assert_eq!(c, c_expected);
}
