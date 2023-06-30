use std::{cell::RefCell, rc::Rc};

use csv_reader::CSVReader;

fn main() {
    let reader = CSVReader::new(String::from("files/test.csv"), true).unwrap();

    println!("Headers");
    println!("{:#?}", reader.headers());
    println!();

    println!("Records:");
    for r in reader.records() {
        println!("{:#?}", r)
    }
}
