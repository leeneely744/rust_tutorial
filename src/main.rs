struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
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