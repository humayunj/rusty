use aggregator::Tweet;

use crate::{
    aggregator::{NewsArticle, Summary},
    trait_bound::Pair,
};

mod aggregator;
mod more_traits;
mod trait_bound;
struct Point<T, U> {
    x: T,
    y: U,
}

impl Point<u32, u32> {
    fn x(&self) -> &u32 {
        &self.x
    }
}

fn main() {
    let number_list = vec![35, 50, 25, 100, 64];
    // println!("{}", largest(&number_list));
    let char_list = vec!['y', 'm', 'a', 'q'];
    // println!("{}", largest_char(&char_list));
    let x = Point { x: 10, y: 2 };
    println!("{}", x.x());

    let tweet = Tweet {
        content: String::from("Hello, World!"),
        reply: false,
        retweet: false,
        username: String::from("humayun219"),
    };
    println!("{}", tweet);
    println!(
        "{}",
        NewsArticle {
            author: String::from("Humayun J"),
            contents: String::from("Yolo"),
            headline: String::from("HEAD"),
            location: String::from("Earth")
        }
    );
    more_traits::main_fn()
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
