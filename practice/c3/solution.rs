struct PrintableStruct {
    a: i32,
    b: String,
}

// custom formatter
impl std::fmt::Display for PrintableStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a:{} b:{}", self.a, self.b)
    } 
}

fn main() {
    // positional parameters
    println!("{1} {2} {0}", 0, 1, 2);

    // named parameters
    println!("my name is: {name}", name="kongjun");

    // width
    println!("Hello {:10}", "Rust");

    // width and positional parameters
    println!("Hello {1:0$}", 10, "Rust");

    // width and named parameters
    println!("Hello {name:width$}", name = "Rust", width = 10);

    // Fill / Allignment
    println!("Hello {:->10}", "Rust");

    // sign
    println!("the grade of Rust: {:+}", 100);

    // # 
    println!("Debug trait for Option: {:#?}", Some(100));
    
    println!("add prefix for octal: {:#o}", 0o11111);

    // precision
    println!("Hello {} is {number:.2}", "x",[number = 0.0001);

    // Display trait
    println!("PrintableStruct: {}", PrintableStruct{a: 1, b: String::from("hello world")});

}
