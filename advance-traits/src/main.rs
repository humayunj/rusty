use core::fmt;
use std::ops::{Add, Deref};

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

struct Counter {
    pub current: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        return None;
    }
}

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let c = Counter { current: 0 };
    for i in c {
        println!("i:{}", i);
    }

    assert_eq!(
        Point { x: 0, y: 1 } + Point { x: 2, y: 2 },
        Point { x: 2, y: 3 }
    );
    let person = Human;
    Pilot::fly(&person);

    let w = Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("w = {}", w);
    println!("Len: {}", w.len());

    add_twice(add_one, 10);

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

enum Status {
    Value(u32),
    Stop,
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
fn add_one(x: i32) -> i32 {
    x + 1
}
fn add_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x) + f(x)
}
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
