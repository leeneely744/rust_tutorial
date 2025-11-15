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

    // byte index 4 is not a char boundary; it is inside 'ん' (bytes 3..6) of `こんにちは、ぼくドラえもんです。`
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\hello-rust.exe` (exit code: 101)
    let hello = "こんにちは、ぼくドラえもんです。";
    // let h = &hello[0..4];
    // println!("{h}");
    for c in hello.chars() {  // explicit characters or bytes
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
}
