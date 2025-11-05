fn main() {
    let mut v: Vec<i32> = Vec::new();
    println!("v is {:?}", v);

    v.push(1);
    let test = &v[0];
    println!("test is {test}");
    
    v.push(2);

    // This is error: mutable borrow occurs here.
    // immutable borrow later used here.
    // println!("test is {test}");

    println!("---------------------------");

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

    println!("---------------------------");

    let v3 = vec![1,2,3];
    for i in &v3 {
        println!("{i}");
    }
    let mut v4 = vec![4,5,6];
    for i in &mut v4 {
        *i += 10;
        println!("{i}");
    }
}

