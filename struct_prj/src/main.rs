struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height 
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("584782297@qq.com"),
        username: String::from("huangliang"),
        active: true,
        sign_in_count: 1,
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);
    let rect = Rectangle {
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
    let sq = Rectangle::square(3);

    user1.email = String::from("hl584782297@gmail.com");

    println!("{}", user1.email);
    // println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    // println!("The area of the rectangle is {} square pixels.", area(rect1));
    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("rect is {:?}", rect);
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

/* fn area(width: u32, height: u32) -> u32 {
    width * height
} */

// fn area(dimemsions: (u32, u32)) -> u32{
//     dimemsions.0 * dimemsions.1
// }