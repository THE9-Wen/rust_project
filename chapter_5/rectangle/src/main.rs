fn main() {
    let rect = Rectangle{
        width: 10,
        height: 20,
    };
    println!("Rect is: {:?}, area is :{}", rect,  rect.area());

    let rect2 = Rectangle {
        width: 5,
        height: 10,
    };
    println!("rect can hold rect2? {}", rect.can_hold(&rect2));

    let square = Rectangle::square(12);
    println!("Square is {:?}", square);
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
