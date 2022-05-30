#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // Takes &self and reference to other Rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function (does not take &self as param)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Also valid, but does not make sense here
impl Rectangle {
    fn can_hold_2(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
     let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
       println!("The rectangle has a nonzero width; it is {}", rect1.width);
   }

   println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
   println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

   let sq = Rectangle::square(3);
   println!("Square: {:?}", sq);
}