#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height:size
        }
    }
}

fn main() {
    let rect = Rectangle{
        width: 30,
        height: 50
    };

    let rect1 = Rectangle{
        width: 20,
        height: 40
    };

    let rect2 = Rectangle{
        width: 40,
        height: 60
    };

    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rect: {:#?}", rect);

    println!("The area of the REctangle is {} square pixels.", rect.area());
}