use core::fmt;

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce(&self, announcement: &str) -> &str {
        print!("{}", announcement);
        self.part
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: fmt::Display,
{
    println!("Announcment: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let a = String::from("hellooo");

    let s: &'static str = "hello";
    // let e;
    // let x;
    {
        let b = String::from("world!");
        // e = ImportantExcerpt { part: b.as_str() }
        // x = longest(a.as_str(), b.as_str());
    } // b goes out of scope
      // println!("{}", e.part)
      // println!("{}", x);
}
