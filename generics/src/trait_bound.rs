use std::fmt::Display;

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x)
        } else {
            println!("The largest number is y = {}", self.y)
        }
    }
}
