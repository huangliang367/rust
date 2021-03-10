const MAX_POINTS: u32 = 100_000;

fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut x = 5;
    let guess: u32 = "32".parse().expect("Not a number!");
    let y = 2.0;
    let z: f32 = 3.0;
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (t1, t2, t3) = tup;
    let fv = five();
    let months = ["January", "February", "March",
    "April", "May", "June",
    "July", "August", "September",
    "October", "November", "December"];

    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
    println!("The value of t1 is: {}", t1);
    another_function(6);
    println!("five() -> i32: {}", fv);
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn if_branch() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 4, 3, or 2")
    }
}

fn loop_test() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
}