use std::io;
fn min_two(a: i32, b: i32) -> i32 {
    if a < b {
        a
    }
    else {
        b
    }
}

fn min(vec: &[i32]) -> i32 {
    if vec.len() == 1 {
        return vec[0];
    }
    let mid = vec.len() / 2;
    min_two(min(&vec[..mid]), min(&vec[mid..]))
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let vec: Vec<i32> = buf.split_whitespace().map(|str| str.parse::<i32>().unwrap()).collect();
    println!("{}", min(&vec[..]));
}
