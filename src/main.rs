enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("123.1.1.1"));
    println!("{}", home);

    let some_number = Some(5);
    let some_char = Some('a');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // error: no implementation for `i8 + Option<{integer}>`


}
