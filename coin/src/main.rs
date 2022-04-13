/*
#[derive(Debug)]
#[warn(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
#[warn(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
*/

/*

fn main() {
    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

  
}
fn add_fancy_hat() { println!("add_fancy_hat")}
fn remove_fancy_hat() {println!("remove_fancy_hat")}


*/

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}


