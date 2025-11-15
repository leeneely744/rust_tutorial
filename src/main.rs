fn main() {
    // let mut s = String::new();
    let data = "initial contents";
    // `String::from` is same as `to_string`
    let mut s = data.to_string();
    let mut s2 = "init".to_string();
    // `add` converts &String to &str.
    let s3 = s + &s2;
    println!("{}", s3);
    
    let mut foo = String::from("foo");
    let bar = "bar";
    foo.push_str(bar);
    println!("{} and {}", foo, bar);

    let a = "tic";
    let b = "toc";
    let c = "toe";
    let abc = format!("{a}-{b}-{c}");
    println!("{abc}");
}
