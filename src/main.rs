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
    // This is not shallow copy, this is 'move'
    let s2 = s1;
    // println!("{}", s1);  // error: value borrowed here after move
    println!("{}", s2);

    let ori = String::from("aaa");
    let clo = ori.clone();  // clone is high cost.
    println!("ori is {}, and clo is {}", ori, clo);

    let cl = String::from("hello");
    let len = calculate_length(&cl);
    println!("The length of '{cl} is {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
