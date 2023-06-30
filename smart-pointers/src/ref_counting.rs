enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use self::List::{Cons, Nil};

pub fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Ref count {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("Ref count after b {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Ref count after c {}", Rc::strong_count(&a));
    }
    println!("Ref count after c ends {}", Rc::strong_count(&a));
}
