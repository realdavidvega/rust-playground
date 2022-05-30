use crate::UsState::Alabama;

#[derive(Debug)] // so we can inspect the state in a minute
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

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn main() {
    let penny: Coin = Coin::Penny;
    penny.value_in_cents();

    let quarter: Coin = Coin::Quarter(UsState::Alaska);
    quarter.value_in_cents();

    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Catch-all Patterns and the _ Placeholder
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // Same as
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // Same as
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // Concise Control Flow with if let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Same as
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // Another example
    let coin: Coin = Coin::Dime;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Same as
    let coin: Coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

// Matches are exhaustive, you need to define both None and Some for Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}