// use std::cmp::Ordering;
// use std::io;
use std::any::type_name;

// use rand::Rng;

fn get_type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let sum = 5 + 10;
    println!("{sum} is {}", get_type_of(&sum));

    let diff = 43.2 - 5.2;
    println!("{diff} is {}", get_type_of(&diff));

    // println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1..=100);
    // // println!("The secret number is: {secret_number}");
    
    // loop {
    //     println!("Please input your guess.");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    
    //     println!("You gesses: {guess}");

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win");
    //             break;
    //         }
    //     }
    // }
}