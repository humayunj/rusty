use core::panic;
use std::cell::RefCell;

struct Test {
    list: RefCell<Vec<String>>,
}

impl Test {
    fn push(&self, v: String) {
        // let l = self.list.borrow();
        self.list.borrow_mut().push(v.clone());
        // println!("Length: {}", l.len());
    }
}

pub fn main() {
    let test = Test {
        list: RefCell::new(vec![]),
    };

    test.push(String::from("Hello"));
}
