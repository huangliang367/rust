const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    let guess: u32 = "32".parse().expect("Not a number!");
    let y = 2.0;
    let z: f32 = 3.0;
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (t1, t2, t3) = tup;

    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
    println!("The value of t1 is: {}", t1);
}
