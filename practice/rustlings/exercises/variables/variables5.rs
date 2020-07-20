// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)


fn main() {
    let number = "3";
    println!("Number {}", number);
    // number在上面被推断为&str，所以下面 number 不可以被修改为整数;
    // 而且number是只读的，不可以被修改。
    println!("Number {}", number);
}
