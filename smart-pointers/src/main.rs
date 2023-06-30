mod cleanup;
mod mocking;
mod ref_counting;
mod ref_counting_mutable;
mod ref_cycle;
mod refcell;
mod refcell_and_rc;
mod weak_ref;

use std::ops::Deref;

use self::List::{Cons, Nil};
// enum List {
//     Cons(i32,List),
//     Nil,
// }
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        println!("deref");
        &self.0
    }
}
fn hello(name: &MyBox<String>) {
    println!("Hello, {}", *(*name))
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("b = {}", list);
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    cleanup::main();
    ref_counting::main();
    refcell::main();
    ref_counting_mutable::main();
    refcell_and_rc::main();
    ref_cycle::main();
    weak_ref::main();
}
