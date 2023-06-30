use std::{cell::RefCell, rc::Rc};

pub fn main() {
    let val = RefCell::new(10);

    let a = Rc::new(val);
    let b = a.clone();
    let c = a.clone();

    // let mut b_ref = b.borrow_mut();
    // let mut c_ref = c.borrow_mut();
    // *b_ref += 2;
    // *c_ref += 8;

    *b.borrow_mut() += 2;
    *c.borrow_mut() += 8;
    println!("Value of a {}", a.borrow());
    println!("Value of b {}", b.borrow());
    println!("Value of c {}", c.borrow());
}
