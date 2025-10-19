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

    println!("------------------");

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1);  // error: value borrowed here after move
    println!("{}", s2);

}