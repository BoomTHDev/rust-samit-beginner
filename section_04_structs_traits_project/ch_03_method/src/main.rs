struct Rectangle {
    width: u32,
    height: u32,
}

// &self คือ ยืมอ่าน
// &mut seld คือ ยืมและเขียน
// no self คือ new struct ออกไป
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[allow(dead_code)]
fn main_demo() {
    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("The area of rectangle is {}", rect.area());

    rect.double_size();
    println!("Rectangle width: {}", rect.width);
    println!("Rectangle height: {}", rect.height);

    let square = Rectangle::square(50);
    println!("New square width: {}", square.width);
    println!("New square height: {}", square.height);
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("The perimeter of rectangle is {}", rect.perimeter());

    let rect2 = Rectangle {
        width: 5,
        height: 15,
    };

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
}
