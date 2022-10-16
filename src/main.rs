/*This example uses a struct to calculate the area of a rectangle 
the struct is unique because it has a method that does the area calculation*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width *  self.height
    }
}

fn main() { 
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels", rect1.area());
}
