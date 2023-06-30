use std::{borrow::Borrow, cell::RefCell, ops::Deref, rc::Rc, vec};

fn main() {
    let mut v = RefCell::new(vec![1]);
    {
        let mut x = v.borrow_mut();
        x.push(10);
    }
    let mut y = v.borrow_mut();

    y.push(20);

    println!("{:#?}", y);
}
