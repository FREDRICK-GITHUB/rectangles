/*This example has more than one method for the given struct
we are checking if the first rectangle can hold the other given rectangles*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width *  self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() { 
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};


    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("This is because the areas of the rectangles is r1: {}, r2: {}, r3: {}",rect1.area(), rect2.area(), rect3.area());
}
