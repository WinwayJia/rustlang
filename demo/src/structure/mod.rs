pub mod enum_demo;

pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

pub fn struct_demo() {
    let user = User {
        username: "someone".to_string(),
        email: String::from("someone@demo.com"),
        sign_in_count: 1,
        active: true,
    };
    let user2 = User {
        username: "user2".to_string(),
        ..user
    };

    let black = Color(0, 0, 0);

    println!(
        "user2.username: {} user2.email: {}",
        user2.username, user2.email,
    );

    let rec = &Rectangle {
        width: 3.2,
        height: 9.3,
    };
    println!(
        "rect is {:?} {:#?} area={} {} can_hold: {}",
        rec,
        rec,
        area(&rec),
        rec.area(),
        rec.can_hold(rec)
    );
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    // invisiable: bool,
}

fn area(rec: &Rectangle) -> f32 {
    rec.height * rec.width
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }
    pub fn area(&self) -> f32 {
        self.height * self.width
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn square(size: f32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
