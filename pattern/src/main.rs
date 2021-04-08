enum IpAddrVersion {
    V4,
    V6,
}

struct IpAddr {
    version: IpAddrVersion,
    address: String,
}

enum IpAddrEnum {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let four = IpAddrVersion::V4;
    let six = IpAddrVersion::V6;

    let home = IpAddr {
        version: IpAddrVersion::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        version: IpAddrVersion::V6,
        address: String::from("::1"),
    };

    let home_enum = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback_enum = IpAddrEnum::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    m.call();

    println!("Hello, world!");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}