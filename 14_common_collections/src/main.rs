use std::collections::HashMap;

fn main() {
    // Vectors
    let _v: Vec<i32> = Vec::new();

    // Macro
    let _v = vec![1, 2, 3];

    // Needs to be mutable
    let mut _v = Vec::new();
    _v.push(5);
    _v.push(6);
    _v.push(7);
    _v.push(8);

    // Dropping
    {
        let _v = vec![1, 2, 3, 4];
        // do stuff with _v
    } // <- v goes out of scope and is freed here

    // Reading elements
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The 3rd element is {}", third);

    // With match, to avoid program panicking
    match v.get(1) {
        Some(third) => println!("The 2nd element is {}", third),
        None => println!("There is no 2nd element."),
    }

    // Interacting with values of a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // deference
        *i += 50;
        println!("Number: {}", i);
    }

    // Use with Enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Strings
    // Create new string
    let mut _s = String::new();

    // From existing data
    let data = "initial contents";

    let s = data.to_string();
    print!("{} \n", s);

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{}", s);

    // Equivalent
    let s = String::from("initial contents");
    println!("{}", s);

    // Strings are UTF-8 encoded
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // Updating a String
    let mut s = String::from("initial contents");
    s = s + ", more contents";
    println!("{}", s);

    // Appending
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    // Pushes a string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // Adds single character
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    // Moving
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    // Getting more complex
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    // Much easier to read, hooman!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // Rust does not support indexing on Strings
    //  let s1 = String::from("hello");
    //  let h = s1[0];

    // Slicing
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // Charts
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // Hash maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Other way, from vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut _scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Accessing
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        None => {}
        Some(value) => println!("Blue match: {}", value)
    }

    // Iterate
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overriding
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Update value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
