fn main() {
    println!("Hello, world!");

    // Functions
    another_function();
    another_function_two(5);
    print_labeled_measurement(5, 'h');

    // Statement
    let _y = 6;

    // Error: statements do not return a value
    //let x = (let y = 6);

    // Expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // Function with return value
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    // if Expressions
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Error: if expects a Bool
    //let number = 3;
    //if number {
    //    println!("number was three");
    // }

    // Correct
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    // Else if
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Error: variables must be single type
    //let condition = true;
    //let number = if condition { 5 } else { "six" };
    //println!("The value of number is: {}", number);

    // Loops

    // Infinite loop
    //loop {
    //    println!("again!");
    //}

    // Nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // Returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Conditional loops (while)
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping through a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // Concise alternative to loop
    let a = [50, 40, 30, 20, 10];
    for element in a {
        println!("the value is: {}", element);
    }

    // For loop and reverse the range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_two(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Error: statements do not return a value
//fn plus_one(x: i32) -> i32 {
//    x + 1;
//}