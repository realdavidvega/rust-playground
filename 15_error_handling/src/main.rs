use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::error::Error;

fn main() {
    // Panic
    //panic!("crash and burn");

    // Will Panic
    //let v = vec![1, 2, 3];
    //v[99];

    // To enable backtrace
    // RUST_BACKTRACE=1 cargo run

    // Using Result for error handling
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Alternative using closures and unwrap_or_else
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // This will call panic! for us
    let f = File::open("hello.txt").unwrap();

    // Using expect with custom message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // This will fail as the return type of main is not a Result
    //let f = File::open("hello.txt")?;
}

// Recoverable errors with Result
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Propagating the error
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Much easier to propagate the error, by just using ? operator
fn read_username_from_file_easier() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Even easier
fn read_username_from_file_even_easier() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// With this standard library, it opens the file and reads it to us, and we return the result
fn read_username_from_file_easiest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// We can also use ? on Option on a similar way
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// This main function would work, as now it has Result return type
fn main_2() -> Result<(), Box<dyn Error>> { // any kind of error
    let f = File::open("hello.txt")?;
    Ok(())
}

// Example of guess game, but using i32 to allow potentially negative numbers
// also we check if the guess is in range
fn guess() {
    loop {
        // --snip--
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
        }
    }
}

// Creating custom type for validation instead of the loop
pub struct Guess {
    value: i32,
}

impl Guess {
    // New function panics if the number is not in the range
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // Getter
    pub fn value(&self) -> i32 {
        self.value
    }
}