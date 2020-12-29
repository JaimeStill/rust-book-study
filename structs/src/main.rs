#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("rect1 is: {:?}", rect1);
    println!("area(&rect1): {}", area(&rect1));
    println!();

    let rect2 = create_rect(18, 32);
    println!("rect2 is: {:#?}", rect2);
    println!("rect2.area(): {}", rect2.area());
    println!();

    let rect2 = clone_new_width(rect2, 24);
    println!("rect2 updated: {:#?}", rect2);
    println!();

    let sq = Rectangle::square(24);
    println!("sq is: {:#?}", sq);
    println!("sq.area(): {}", sq.area());
    println!();

    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1? {}", rect2.can_hold(&rect1));
    println!("rect1 can hold sq? {}", rect1.can_hold(&sq));
    println!("rect2 can hold sq? {}", rect2.can_hold(&sq));
    println!();
}

fn create_rect(width: u32, height: u32) -> Rectangle {
    Rectangle {
        width,
        height
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn clone_new_width(rect: Rectangle, width: u32) -> Rectangle {
    Rectangle {
        width,
        ..rect
    }
}