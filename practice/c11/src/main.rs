//! # 笨方法学C 练习11
//! ```C
//! #include <stdio.h>
//! int main(int argc, char *argv[])
//! {
//!     // go through each string in argv
//!     int i = 0;
//!     while(i < argc) {
//!         printf("arg %d: %s\n", i, argv[i]);
//!         i++;
//!     }
//!
//!     // let's make our own array of strings
//!     char *states[] = {
//!         "California", "Oregon",
//!         "Washington", "Texas"
//!     };
//!     int num_states = 4;
//!     i = 0;  // watch for this
//!     while(i < num_states) {
//!         printf("state %d: %s\n", i, states[i]);
//!         i++;
//!     }
//!     return 0;
//! }
//! ```
use std::env;
fn main() {
	let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        println!("arg {}: {}", i, arg);
    }

    let states = vec!["California", "Oregon",
                      "Washington", "Texas"];
    for (i, arg) in states.iter().enumerate() {
        println!("state {}: {}", i, arg);
    }
}
