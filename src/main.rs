fn main() {
    let v: Vec<i32> = Vec::new();
    println!("v is {:?}", v);

    // vec! is macro
    let mut v2 = vec![1, 2, 3];
    println!("v2 is {:?}", v2);
    // mut is must
    v2.push(4);
    println!("v2 is {:?}", v2);
}

