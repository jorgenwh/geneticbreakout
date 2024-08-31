mod mat;

fn main() {
    println!("Hello, world!");
    let m = mat::Matrix::new(2, 2);

    println!("m.get(0)=={}", m.get(0, 0));
}
