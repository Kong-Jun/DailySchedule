use std::io;
use std::cmp::PartialOrd;
fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    if n == 1 {
        print!("{}", buf.trim().parse::<u128>().unwrap());
        return ();
    }
    let mut vec: Vec<&str> = buf.trim().split_whitespace().collect();
    
    vec.sort_by(|a, b| b.partial_cmp(a).unwrap());
    
    let mut output = String::from(vec[0]);

    for i in 1..n {
        let str = vec[i].clone();
        output.push_str(&str);
    }

    print!("{}", output.parse::<u128>().unwrap());
    
}
