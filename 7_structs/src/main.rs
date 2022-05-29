// Structs
struct User {
    // fields
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    // Instance of User (mutable)
    let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
    };
    println!("{}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    // Built from a function, creating instances from other instances
    let user2 = build_user(user1.email, user1.username);
    println!("{}, {}", user2.email, user2.username);

    // Struct update syntax
    let user3 = User {
        email: String::from("another3@example.com"),
        ..user2
    };
    println!("{}, {}", user3.email, user3.username);

    // Using Tuple Structs without Named Fields to Create Different Types
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields
    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // Using the Field Init Shorthand
        username, // Using the Field Init Shorthand
        active: true,
        sign_in_count: 1,
    }
}