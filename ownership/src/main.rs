fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    let len = calculate_length(&s);

    let s2 = s.clone();

    println!("s = {}, s1 = {}", s, s2);
    println!("The length of {} is {}", s, len);
    takes_ownership(s);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}