use std::io;
fn main()
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let vec:Vec<i32> = input.split_whitespace().map(|str| str.parse::<i32>().unwrap()).collect();
    let sum: i32 = vec.iter().sum();
    println!("Sum = {}; Average = {:.1}", sum , (sum as f64) / (vec.len() as f64));
}
