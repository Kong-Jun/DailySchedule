// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

// 原来这个函数在main函数下面，而且只有参数名，没有类型

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
