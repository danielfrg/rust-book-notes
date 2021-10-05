#![allow(unused)]

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Match

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Lucky Dime!");
            1
        }
        Coin::Quarter => 25,
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}", four);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    let msg1 = Message::Quit;
    let msg2 = Message::Write(String::from("message"));
    let msg3 = Message::Move { x: 1, y: 3 };

    println!("{:?}", msg1);
    println!("{:?}", msg2);
    println!("{:?}", msg3);

    let some_number = Some(5);
    println!("{:?}", some_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // This is not possible
    // let sum = x + y;

    // Convert the Some to T using expect()
    let y: i8 = Some(5).expect("Failed to read line");
    let sum = x + y;
    println!("Sum {}", sum);

    // match operator

    let coin = Coin::Penny;
    println!("A penny: {}", value_in_cents(coin));
    println!("A dime: {}", value_in_cents(Coin::Dime));

    println!(
        "A quarter: {}",
        value_in_cents2(Coin2::Quarter(UsState::Alabama))
    );

    //
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // If let
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("tres!")
    }
}
