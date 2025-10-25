struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// It enable Rectangle to use {rect1:?}
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
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect2 = Rectangle {
        width: 40,
        height: 350,
    };
    println!(
        "The area of the rect2 is {}.",
        rect2.area()
    );

    println!("------------------");

    // let rect1 = (30, 50);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)  // borrow the struct
    );

    // 'main' retains its ownership and continue using rect1.
    // println!("width is {}, height is {}", rect1.width, rect1.height);
    println!("rect1 is {rect1:?}");

    dbg!(&rect1); // dbg! returns ownership of the expression's value

    println!("rect2 can hold rect1? -> {}", rect2.can_hold(&rect1));
    
    println!("------------------");

    let user1 = User {
        active: true,
        username: String::from("someone"),
        email: String::from("test@adf.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.username);

    let user2 = User {
        email: String::from("asdf@sdfi.sdof"),
        ..user1  // must come last to specify that any remaining fields should get their values from the corresponding fields in user1.
    };

    // error! value borrowed here after move
    // println!("{}", user1.username);
}

fn area(rectangle: &Rectangle) -> u32 {
// fn area(dimensions: (u32, u32)) -> u32 {
    // dimensions.0 * dimensions.1
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    // The entire instance must be mutable
    // Because the email field and the email parameter have the same name, we only need to write email.
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}