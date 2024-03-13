
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

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
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn area(dimension: &Rectangle) -> u32 {
    dimension.width * dimension.height
}

fn main() {
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("user@mail.com"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("user2");

    let user2 = build_user(
        String::from("zhuravkovigor@icloud.com"),
        String::from("Igor")
    );

    let user3 = User {
        email: String::from("james@mail.com"),
        username: String::from("James"),
        ..user2
    };

    let square = Rectangle::square(3);

    println!("{:#?}", square);
    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 60,
    };

    println!("{:#?}", rect);

    println!("The area of the rectangle is {} square pixels.", area(&rect));
    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
}



fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
