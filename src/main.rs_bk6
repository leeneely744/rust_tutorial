enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("123.1.1.1"));
    // println!("{}", home);

    let some_number = Some(5);
    let some_char = Some('a');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // error: no implementation for `i8 + Option<{integer}>`

    value_in_cents(Coin::Quarter(UsState::Alabama));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // snip
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1819,
        }
    }
}

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     if let Coin::Quarter(state) = coin {
//         if state.existed_in(1900) {
//             Some(format!("{state:?} is pretty old"))
//         } else {
//             Some(format!("{state:?} is relativel new."))
//         }
//     } else {
//         None
//     }
// }

// same above
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old"))
    } else {
        Some(format!("{state:?} is relativel new."))
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),  // variant
}

fn value_in_cents(coin: Coin) -> u8 {
    // the arms' patterns must cover all possibilities.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(state) => {
            println!{"State quarter from {state:?}!"};
            25
        }
    }
}