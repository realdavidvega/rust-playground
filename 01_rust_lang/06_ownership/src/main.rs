fn main() {
    // Variable scope
    {                            // s is not valid here, it’s not yet declared
        let _s = "hello";   // s is valid from this point forward
        // do stuff with s
    }                           // this scope is now over, and s is no longer valid

    // String type
    let mut s = String::from("string");
    s.push_str(", is a type!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // Move
    let _x = 5;
    let _y = _x;

    let s1 = String::from("first string");
    let s2 = s1;
    // Error: s1 now is out of scope!
    //println!("{}, world!", s1);
    println!("{}, second string!", s2);

    // Clone
    let s1 = String::from("string");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Copy
    let x = 5;
    let y = x;

    // Works as they have fixed size on stack
    println!("x = {}, y = {}", x, y);

    // Ownership and Functions
    let s = String::from("string ownership");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward
    // After main end, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    let _s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("give back ownership");     // s2 comes into scope
    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
    // After main end, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    // References and Borrowing
    let s1 = String::from("borrowing");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Error: you can't change something you borrowed!
    //let s = String::from("hello");
    //change(&s);

    // Mutable References
    let mut s = String::from("change mutable");
    change(&mut s);
    println!("{}", s);

    // Error: you can't have two mutable references at same time!
    //let mut s = String::from("hello");
    //let r1 = &mut s;
    //let r2 = &mut s;
    //println!("{}, {}", r1, r2);

    // Create new scope for multiple mutable references
    let mut s = String::from("new scope");
    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    println!("{}", r2);

    // Error: Can't combine mutable and immutable references
    //let mut s = String::from("hello");
    //let r1 = &s; // no problem
    //let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM
    //println!("{}, {}, and {}", r1, r2, r3);

    // Borrowing ends after the println
    let mut s = String::from("borrowing 2");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References
    // dangling pointer--a pointer that references a location in memory that may have been given to someone else
    // Error: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    //let reference_to_nothing = dangle();

    // This works as we are returning a string, not a reference to a string
    let s = no_dangle();
    println!("{}", s);

    // Slice Type -> a kind of reference, does not have ownership
    let mut s = String::from("a string slice");
    println!("{}", s);
    let word = first_word(&s); // word will get the value 5
    println!("{}", word);
    s.clear(); // this empties the String, making it equal to ""
    println!("empty string: {}", s);
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // Solution? String Slices
    let s = String::from("hello world");
    let len = s.len();
    let hello = &s[0..5]; // same as &s[..5];
    let world = &s[6..len]; // same as &s[6..11] or &s[6..];
    println!("{}", hello);
    println!("{}", world);
    let hello_world = &s[..]; // same as &s[0..len];
    println!("{}", hello_world);

    let slice = first_word_slice(&s);
    println!("{}", slice);

    // Error: can't borrow first as immutable reference on first_world(&s) and then as mutable reference on s.clear()
    //let mut s = String::from("hello world");
    //let word = first_word(&s);
    //s.clear(); // error!
    //println!("the first word is: {}", word);

    // String Literals Are Slices
    let s = "Hello, world!"; //&str type, immutable reference
    println!("{}", s);

    // String Slices as Parameters
    let my_string = String::from("first second");
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_slice_str(&my_string[0..6]);
    println!("{}", word);
    let word = first_word_slice_str(&my_string[..]);
    println!("{}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_slice_str(&my_string);
    println!("{}", word);
    let my_string_literal = "third fourth";
    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_slice_str(&my_string_literal[0..6]);
    println!("{}", word);
    let word = first_word_slice_str(&my_string_literal[..]);
    println!("{}", word);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice_str(my_string_literal);
    println!("{}", word);

    // Other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
   // it refers to, nothing happens.

// Error: you can't change something you borrowed!
//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

fn change(some_string: &mut String) {
    some_string.push_str(", here");
}

// Error: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}

// This works as we are returning a string, not a reference to a string
fn no_dangle() -> String {
    let s = String::from("no dangle example");
    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_slice_str(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}