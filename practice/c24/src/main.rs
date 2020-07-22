//! # 笨方法学C 练习24
//! ```
//! #include <stdio.h>
//! #include "dbg.h"
//! #define MAX_DATA 100
//!
//! typedef enum EyeColor {
//!     BLUE_EYES, GREEN_EYES, BROWN_EYES,
//! 
//!     BLACK_EYES, OTHER_EYES
//! 
//! } EyeColor;
//! 
//! const char *EYE_COLOR_NAMES[] = {
//!     "Blue", "Green", "Brown", "Black", "Other"
//! };
//! 
//! typedef struct Person {
//!     int age;
//!     char first_name[MAX_DATA];
//!     char last_name[MAX_DATA];
//!     EyeColor eyes;
//!     float income;
//! } Person;
//! 
//! int main(int argc, char *argv[])
//! {
//!     Person you = {.age = 0};
//!     int i = 0;
//!     char *in = NULL;
//! 
//!     printf("What's your First Name? ");
//!     in = fgets(you.first_name, MAX_DATA-1, stdin);
//!     check(in != NULL, "Failed to read first name.");
//!
//!     printf("What's your Last Name? ");
//!     in = fgets(you.last_name, MAX_DATA-1, stdin);
//!     check(in != NULL, "Failed to read last name.");
//!
//!     printf("How old are you? ");
//!     int rc = fscanf(stdin, "%d", &you.age);
//!     check(rc > 0, "You have to enter a number.");
//! 
//!     printf("What color are your eyes:\n");
//!     for(i = 0; i <= OTHER_EYES; i++) {
//!         printf("%d) %s\n", i+1, EYE_COLOR_NAMES[i]);
//!     }
//!     printf("> ");
//!     int eyes = -1;
//!     rc = fscanf(stdin, "%d", &eyes);
//!     check(rc > 0, "You have to enter a number.");
//! 
//!     you.eyes = eyes - 1;
//!     check(you.eyes <= OTHER_EYES && you.eyes >= 0, "Do it right, that's not an option.");
//! 
//!     printf("How much do you make an hour? ");
//!     rc = fscanf(stdin, "%f", &you.income);
//!     check(rc > 0, "Enter a floating point number.");
//!     printf("----- RESULTS -----\n");
//!     printf("First Name: %s", you.first_name);
//!     printf("Last Name: %s", you.last_name);
//!     printf("Age: %d\n", you.age);
//!     printf("Eyes: %s\n", EYE_COLOR_NAMES[you.eyes]);
//!     printf("Income: %f\n", you.income);
//! 
//!     return 0;
//! 
//! error:
//! 
//! 
//! 
//!     return -1;
//! 
//! }
use std::default::Default;
use std::io;
use std::fmt::Display;


enum EyeColor {
    BuleEyes,
    GreenEyes,
    BrownEyes,
    BlackEyes,
    OtherEyes,
}

impl Default for EyeColor {
    fn default() -> Self {
        EyeColor::BlackEyes
    }
}

#[derive(Default)]
struct Person {
    age: u32,
    first_name: String,
    last_name: String,
    eyes: EyeColor,
    income: f64,
}

const EYE_COLOR_NAMES: [&'static str; 5] = [
    "Blue", "Green", "Brown", "Black", "Other",
    ];

#[allow(unused)]
fn main() -> std::io::Result<()> {
    println!("What's your First name");
    let mut first_neme = String::new();
    io::stdin().read_line(&mut first_neme)?;
    
    println!("What's your last name?");
    let mut last_name = String::new();
    io::stdin().read_line(&mut last_name)?;
    
    println!("How old are you?");
    let mut old_str = String::new();
    io::stdin().read_line(&mut old_str)?;
    let old: u32 = old_str.trim().parse().unwrap(); 

    println!("What color are your eyes:\n");
    let mut i = 0;
    for eyes_color in &EYE_COLOR_NAMES {
        i += 1;
        println!("{}: {}", i, eyes_color);
    }
    println!(">");
    let mut i_buf = String::new();
    io::stdin().read_line(&mut i_buf)?;
    let i = i_buf.trim().parse::<usize>().unwrap();
    if i == 0 {
        eprintln!("input number must bigger than 0");
        return Err(io::Error::from(io::ErrorKind::Other));
    }
    let eyes_color = match EYE_COLOR_NAMES[i - 1] {
        "Blue" => EyeColor::BuleEyes,
        "Green" => EyeColor::GreenEyes,
        "Brown" => EyeColor::BrownEyes,
        "Black" => EyeColor::BlackEyes,
        "Other" => EyeColor::OtherEyes,
        _ => panic!("Error in EYE_COLOR_NAMES"),
    };
    
    println!("How much do you make an hour?");
    let mut income = String::new();
    io::stdin().read_line(&mut income)?;
    let income = income.trim().parse::<f64>().unwrap();

    let person = Person {
        first_name: first_neme,
        last_name: last_name,
        age: old,
        eyes: eyes_color,
        income: income,
    };
    
    println!("{:-^30}", "RESULT");
    println!("Frist Name: {}", person.first_name);
    println!("Last Name: {}", person.last_name);
    println!("Age: {}", person.age);
    println!("Eye Color: {}", EYE_COLOR_NAMES[person.eyes as usize]); 
    println!("Income: {:.0}", person.income);

    Ok(())
}
