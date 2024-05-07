use concurrency::martix::Martix;

fn main() {
    let a = Martix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    let b = Martix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
    let c = a * b;
    println!(" a * b = {}", c);
}
