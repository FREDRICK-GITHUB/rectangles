/*This example has more than one method for the given struct
we are checking if the first rectangle can hold the other given rectangles*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// a struct can have more than one impl block
impl Rectangle {
    fn area(&self) -> u32 {
        self.width *  self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}

fn main() { 
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};
    let rect4 =  Rectangle::square(30);


    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("This is because the areas of the rectangles is r1: {}, r2: {}, r3: {}",rect1.area(), rect2.area(), rect3.area());
    println!("The size of rect4 is: {:?}", &rect4);
}
