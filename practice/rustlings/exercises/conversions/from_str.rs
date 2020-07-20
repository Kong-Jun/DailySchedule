// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// Steps:
// 1. If the length of the provided string is 0, then return an error
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return an error
// Otherwise, then return a Result of a Person object
impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() < 1 {
            return Err(String::from("argument is too short"));
        }
        let idx = s.find(',');
        if idx.is_none() {
            return Err(String::from("comma is not found"));
        }
        let idx = idx.unwrap();
        
        let (name, mut age) = s.split_at(idx);
        if age.len() > 1 {
            age = &age[1..];
        } else {
            return Err(String::from("age is not found"));
        }
        eprintln!("name: {}\t age:{}", name, age);
        let age = age.parse::<usize>();
        if age.is_err() {
            return Err(String::from("error ocuur when parse age"));
        }
        
        let age = age.unwrap();
        Ok(Person {
            name: String::from(name),
            age: age,
        }
        )

    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        assert!("John,32".parse::<Person>().is_ok());
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John".parse::<Person>().unwrap();
    }
}
