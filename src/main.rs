fn main() {
    let v: Vec<i32> = Vec::new();
    println!("v is {:?}", v);

    // vec! is macro
    let mut v2 = vec![10, 20, 30];
    println!("v2 is {:?}", v2);
    // mut is must
    v2.push(40);
    println!("v2 is {:?}", v2);
    
    let third = v2[2];
    println!("third is {third}");
    println!("v2 is {:?}", v2);

    // error
    // let non = v2[5];

    let number: Option<&i32> = v.get(5);
    match number {
        Some(number) => println!("number is {number}"),
        None => println!("not number"),
    }
}

