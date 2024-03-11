mod base;

fn main() {
    let v = vec![[1, 2, 3], [4, 5, 6]];
    let s = vec![2, 3];
    let mut a = base::Tensor::new(v, s);
    println!("Index out of bounds");
}