/*This example uses a struct to calculate the area of a rectangle */
fn main() { 
    let rect1 = Rectangle { width: 30, height: 50 };
    
    println!("the struct instance we have created is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels", area(&rect1));
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
