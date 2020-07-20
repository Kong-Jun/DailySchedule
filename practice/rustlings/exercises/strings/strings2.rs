// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)


fn main() {
    let word = String::from("green"); // Try not changing this line :)
    // is_a_color_word() 的参数是 &str，需要使用
    // &String，在传递给函数时会发生解引用强制转换变成&str
    // if is_a_color_word(word) {
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
