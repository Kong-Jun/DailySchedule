fn main() {
    let bo = Some(Box::new(5));
    if let Some(b) = bo {
        println!("{}", b);
    }
    println!("{}", bo.unwrap());
}
