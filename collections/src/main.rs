use core::panic;
use std::collections::HashMap;
mod departments;
mod files;
mod mode;
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);

    let third_ref: &i32 = &v[2];

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There's no third element"),
    }

    for i in &v {
        println!("{i}")
    }

    let row = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Float(10.5),
    ];
    for r in row {
        match r {
            SpreadsheetCell::Int(v) => println!("{v}"),
            SpreadsheetCell::Float(v) => println!("{v}"),
            SpreadsheetCell::Text(v) => println!("{v}"),
        }
    }

    let mut s = "init content".to_string();
    s.push('س');

    let mut h = HashMap::new();
    h.insert(String::from("Blue"), 10);
    h.insert(String::from("Red"), 20);
    h.insert(String::from("Green"), 30);

    println!("{}", mode::computeMode(&vec![1, 2, 3, 3, 1, 3]));

    let r = files::read_username_from_file().unwrap();
    println!("{r}");
}
