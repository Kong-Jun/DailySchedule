// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)


fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    // 应该返回一个 i32 整数。加了 ; 号后就返回()，类型不对。
    // num * num;
    num * num
}
