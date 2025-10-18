fn main() {
    println!("main2");
    let a: u32 = "32".parse().expect("Not a number");
    println!(a);
}