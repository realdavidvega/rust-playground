// Enable debug
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Calculate area
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // With tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuples(rect1)
    );

    // With struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    // Print debug :?
    println!("rect1 is {:?}", rect1);

    // Another way of using Debug format
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

// Calculate area
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// With tuples
fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// With struct
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}