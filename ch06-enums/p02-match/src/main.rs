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

fn value_in_cents(coin: Coin) -> u32 {
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
    let a = value_in_cents(Coin::Nickel);
    let b = value_in_cents(Coin::Penny);
    let c = value_in_cents(Coin::Dime);
    let d = value_in_cents(Coin::Quarter(UsState::Alaska));
    let e = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{} {} {} {} {}", a, b, c, d, e);

    /* The _ Placeholder */

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
