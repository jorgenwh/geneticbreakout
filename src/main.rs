mod matrix;

fn main() {
    println!("Hello, world!");

    let mut a = matrix::matrix::Matrix::new(2, 3);
    let mut b = matrix::matrix::Matrix::new(3, 2);

    println!("a={:?}", a);
    println!("b={:?}", b);

    for i in 0..2 {
        for j in 0..3 {
            a.set(i, j, (i * 3 + j) as f32);
            b.set(j, i, (i * 3 + j) as f32);
        }
    }

    println!("a={:?}", a);
    println!("b={:?}", b);

    let c = matrix::ops::matmul(&a, &b);

    println!("c={:?}", c);

    println!("a:\n{}", a);
    println!("b:\n{}", b);
    println!("a:\n{}", c);
}
