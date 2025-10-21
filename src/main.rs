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

    println!("------------------");

    let s4 = String::from("abcdefg hijklmn");
    println!("{}", first_word(&s4));
}

// &str is a slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world"); // error: `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
                              // `mut` is necessary when declaring s and pass `calculate_length(&mut s)`.
    s.len()
}
