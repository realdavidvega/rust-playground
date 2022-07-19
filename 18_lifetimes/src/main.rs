use std::fmt::Display;

// x and y can have different lifetimes and we don't know
// which of them is used as the returned lifetime.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// We need to use the generic lifetime annotation
// They describe the relationship between the lifetimes of
// multiple references and how they relay to each other.

// Now x, y and the return value will have the same lifetime.
// Not creating a lifetime, but making a relationship between them.
// Lifetime of the returned value, will be the smallest of the
// function arguments.
fn longest_generic<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// The lifetime of our return value always has to be tied to
// the lifetime of one of our parameters.
// Because if we pass back a reference from a function, it must
// be a reference of something that we passed in. That's because
// we can't return a reference of something created inside the fn.
fn longest_generic2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Example that does not work because of what we explained above.
// After the function gets over, the local variables get destroyed.
// fn longest_generic3<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really ling string");
//     result.as_str()
// }

// This works because we are returning a type which transfers
// ownership.
fn longest_generic3<'a>(x: &str, y: &str) -> String {
    let result = String::from("really ling string");
    result
}

// Struct example
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime elision
// This works as the compiler can infer the lifetime by
// the lifetime elision rules:

// 1. Each parameter that is a reference gets its own lifetime param.

// 2. If there is exactly one input lifetime param, that lifetime is
// assigned to all output lifetime parameters.

// 3. If there are multiple input lifetime params, byt one of them is
// &self or &mut self the lifetime of self is assigned to all output
// lifetime parameters (only applies to methods).

// The compiler does this:
//fn first_word<'a>(s: &'a str) -> &'a str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Lifetime annotations inside of methods
impl<'a> ImportantExcerpt<'a> {
    // We don't need to add lifetime annotations bcz Rule 3.
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Putting all together
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // x does not live long enough
    // {
    //     let r;          // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;   // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {}", r); //          |
    // }                         // ---------+

    // Now it works
    {
        let x = 5;            // ----------+-- 'b
        //           |
        let r = &x;          // --+-- 'a  |
        //   |       |
        println!("r: {}", r);      //   |       |
        // --+       |
    }                              // ----------+

    // Lifetimes example
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // Won't compile
    //let result = longest(string1.as_str(), string2);
    //println!("The longest string is {}", result);

    // This is valid
    let result = longest_generic(string1.as_str(), string2);

    // The result lifetime is equal to the smallest lifetime of the
    // arguments passed in. In this case, both have the same lifetime.
    println!("The longest string is {}", result);

    // Different lifetimes example
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest_generic(string1.as_str(), string2.as_str());
        println!("The longest string generic is {}", result);
    }

    // This won't work because result is in different scope.
    // This means that the lifetime of result on the println
    // has already ended.
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest_generic(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // This will work as longest_generic2 only has one return
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest_generic2(string1.as_str(), string2.as_str());
    }
    println!("The longest string 2 is {}", result);

    // Struct example
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Static lifetime (has the duration of the program)
    // All string literals have static lifetime, bcz they
    // are stored in the program binary
    let s: &'static str = "I have a static lifetime.";
}