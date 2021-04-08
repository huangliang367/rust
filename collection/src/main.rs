enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];
    println!("The third element if {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &mut v {
        *i += 50;
    }
}
