use std::cmp::Ordering;
use std::io;
use std::any::type_name;

use rand::Rng;

fn main() {
    let mut s = String::from("hello");
    println!("{s}");

    // error
    // s = "Rust";
    // println!("{s}");

    s.push_str(", world");
    println!("{s}");
}