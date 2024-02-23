enum IpAddrKind {
    V4,
    V6,
}

struct IpAddress {
    kind: IpAddrKind,
    address: String,
}

// More concise
enum IpAddress2 {
    V4(String),
    V6(String),
}

// Even more
enum IpAddress3 {
    V4(u8, u8, u8, u8),
    V6(String),
}


struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// Another example
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Equivalent to last one but in struct
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Impl on enums
impl Message {
    fn call(&self) {
        println!("Message!");
    }
}

fn main() {
    let home = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Same type, different variants with enum
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Put data directly inside the enum instead of creating a struct with enum
    let home = IpAddress2::V4(String::from("127.0.0.1"));
    let loopback = IpAddress2::V6(String::from("::1"));

    // Different type of data
    let home = IpAddress3::V4(127, 0, 0, 1);
    let loopback = IpAddress3::V6(String::from("::1"));

    // Call an enum method
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option enum
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // Error: Rust doesn't know how to add an i8 and an Option<i8>, as they are different types!
    //let x: i8 = 5;
    //let y: Option<i8> = Some(5);
    //let sum = x + y;

    if some_number.is_some() {
        println!("Is some!");
    }
}

// We can use both variants here
fn route(ip_kind: IpAddrKind) {}
