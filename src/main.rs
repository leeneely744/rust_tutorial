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
}

fn build_user(email: String, username: String) -> User {
    // The entire instance must be mutable
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}