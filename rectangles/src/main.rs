#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (width1, height1);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect2 is {:#?}", rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(demensions: (u32, u32)) -> u32 {
    demensions.0 * demensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
