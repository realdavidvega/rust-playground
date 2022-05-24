use std::io;

fn main() {
    // Variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of 3 hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let x = 5;
    let x = x + 1;
    println!("The value of x is: {}", x);
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // Mut vs Shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // Error
    //let mut spaces = "   ";
    //spaces = spaces.len();

    // Data Types
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of floored is: {}", guess);

    // Error
    //let guess = "42".parse().expect("Not a number!");

    // Floating-Point Types
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Numeric Operations
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    println!("The value of floored is: {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    // Booleans
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Characters (single-quoted)
    let _c = 'z';
    let _z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    // Tuples
    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    // Accessing Tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    // Arrays
    let _a = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // Array of type i32 and with 5 elements
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Accessing Array Elements
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    // Array with 5 elements of value 3
    let _a = [3; 5];

    // Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
