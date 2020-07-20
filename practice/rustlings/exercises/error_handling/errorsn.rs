// errorsn.rs
// This is a bigger error exercise than the previous ones!
// You can do it! :)
//
// Edit the `read_and_validate` function so that it compiles and
// passes the tests... so many things could go wrong!
//
// - Reading from stdin could produce an io::Error
// - Parsing the input could produce a num::ParseIntError
// - Validating the input could produce a CreationError (defined below)
//
// How can we lump these errors into one general error? That is, what
// type goes where the question marks are, and how do we return
// that type from the body of read_and_validate?
//
// Execute `rustlings hint errorsn` for hints :)


use std::num::ParseIntError;
use std::io::ErrorKind; 
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
enum GenaralError {
    Io(io::Error),
    Parse(ParseIntError),
    Creation(CreationError),
}

impl fmt::Display for GenaralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenaralError::Io(ref err)       => write!(f, "{}", err.to_string()),
            GenaralError::Parse(ref err)    => write!(f, "Parse Error: {}", err),
            GenaralError::Creation(ref err) => write!(f, "Creation Error: {}", err),
        }
    }
}

impl error::Error for GenaralError {
    // description()方法已经过时了，使用std::Display trait
    // fn description(&self) ->&str {
    //     match *self {
    //         GenaralError::Io(ref err)       => err.description(),
    //         GenaralError::Parse(ref err)    => err.description(),
    //         GenaralError::Creation(ref err) => err.description(),
    //     }
    // }
        
    // cause()方法已经过时，定义source()方法
    // fn cause(&self) -> Option<&dyn error::Error> {
    //     match *self {
    //         GenaralError::Io(ref err)       => Some(err),
    //         GenaralError::Parse(ref err)    => Some(err),
    //         GenaralError::Creation(ref err) => Some(err),
    //     }
    // }
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            GenaralError::Io(ref err)       => Some(err),
            GenaralError::Parse(ref err)    => Some(err),
            GenaralError::Creation(ref err) => Some(err),
        }
    }
}

// PositiveNonzeroInteger is a struct defined below the tests.
fn read_and_validate(b: &mut dyn io::BufRead) -> Result<PositiveNonzeroInteger, GenaralError> {
    // let mut line = String::new();
    // b.read_line(&mut line);
    // let num: i64 = line.trim().parse();
    // let answer = PositiveNonzeroInteger::new(num);
    // answer
    let mut line = String::new();
    // read_line()方法的返回值是Result<u32>，因此下面的代码是错的，下面的代码假设read_line()返回值是Result<String,
    // io::error>
    //
    // b.read_line(&mut line).map_err(GenaralError::Io)
    //     .and_then(|line| {
    //         line.trim().parse()
    //         .map_err(GenaralError::Parse)
    //         // .map(|_| line)
    //     })
    // .and_then(|num| {
    //     PositiveNonzeroInteger::new(num).map_err(GenaralError::Creation)
    // })
    if let Err(error) = b.read_line(&mut line) {
        return Err(GenaralError::Io(error));
    }
    
    line.trim().parse().map_err(|err| GenaralError::Parse(err))
        .and_then(|n| PositiveNonzeroInteger::new(n).map_err(|err| GenaralError::Creation(err)))

}

// This is a test helper function that turns a &str into a BufReader.
// fn test_with_str(s: &str) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
fn test_with_str(s: &str) -> Result<PositiveNonzeroInteger, GenaralError> {
    let mut b = io::BufReader::new(s.as_bytes());
    read_and_validate(&mut b)
}

#[test] 
fn test_success() {
    let x = test_with_str("42\n");
    assert_eq!(PositiveNonzeroInteger(42), x.unwrap());
}

#[test]
fn test_not_num() {
    let x = test_with_str("eleven billion\n");
    assert!(x.is_err());
}

#[test]
fn test_non_positive() {
    let x = test_with_str("-40\n");
    assert!(x.is_err());
}

#[test]
fn test_ioerror() {
    struct Broken;
    impl io::Read for Broken {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "uh-oh!"))
        }
    }
    let mut b = io::BufReader::new(Broken);
    assert!(read_and_validate(&mut b).is_err());
    assert_eq!("uh-oh!", read_and_validate(&mut b).unwrap_err().to_string());
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value == 0 {
            Err(CreationError::Zero)
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[test]
fn test_positive_nonzero_integer_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((self as &dyn error::Error).description())
    }
}

impl error::Error for CreationError {
    fn description(&self) -> &str {
        match *self {
            CreationError::Negative => "Negative",
            CreationError::Zero => "Zero",
        }
    }
}
