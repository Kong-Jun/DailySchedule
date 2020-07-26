use std::io;
use std::cmp::{min, max};
macro_rules! make_fun {
    ($fn_name: ident, $called_fn: ident) => (
        fn $fn_name(vec: &[i32]) -> i32 {
            if vec.len() == 1 {
                return vec[0];
            }
            let mid = vec.len() / 2;
            $called_fn($fn_name(&vec[..mid]), $fn_name(&vec[mid..]))
        }
    )
}
make_fun! (vec_min, min);
make_fun! (vec_max, max);

fn main()
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let vec: Vec<i32> = buf.split_whitespace().map(|str| str.parse::<i32>().unwrap()).collect();
    println!("{}", vec_max(&vec[..]) - vec_min(&vec[..]));
}

