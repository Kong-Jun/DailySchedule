// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)


fn main() {
    // call_me 接受一个 i32 参数，但调用时却没有给参数。
    // call_me();
    call_me(32);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
