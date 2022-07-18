//fn largest_i32(list: &[i32]) -> i32 {
fn largest_i32(list: &Vec<i32>) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//fn largest_char(list: &[char]) -> char {
fn largest_char(list: &Vec<char>) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generics
//fn largest<Type, U>(list: Vec<char>) -> char {
// Trait for can be ordered and copied, primitives like integers or chars
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// Generics with enums
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Generics with struct
struct Point {
    x: i32,
    y: i32
}

struct GenericPoint<T> {
    x: T,
    y: T
}

struct GenericPoint2<T, U> {
    x: T,
    y: U
}

// Implementation, any type
impl<U> GenericPoint<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

// Only f64
impl GenericPoint<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

impl<T, U> GenericPoint2<T, U> {
    fn mix<V, W>(self, other: GenericPoint2<V, W>) -> GenericPoint2<T, W> {
        GenericPoint2 {
            x: self.x,
            y: other.y
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    //let result = largest_i32(&number_list);
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    //let result = largest_char(&char_list);
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // Generics
    let result = largest(number_list);
    println!("The largest number is {}", result);
    let result = largest(char_list);
    println!("The largest char is {}", result);

    // Generics structs
    let p1 = Point { x:5, y:10 };

    // Error
    //let p1 = Point { x:5.0, y:10.0 };
    let p2 = GenericPoint2 { x:5.0, y:10 };

    let p1 = GenericPoint {x: 5, y: 10};
    p1.x();

    let p2 = GenericPoint {x: 5.0, y: 10.0};
    p2.x();
    p2.y();

    // Mix generics
    let p1 = GenericPoint2 { x:5, y:10.4 };
    let p2 = GenericPoint2 { x:"Hello", y:'c' };
    let p3 = p1.mix(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Rust coverts this to two different ones, so its good performance
    let integer = Option::Some(5);
    let float = Option::Some(5.0);
}