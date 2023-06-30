use std::vec;

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using {} as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as background");
        } else {
            println!("Using orange as background");
        }
    } else {
        println!("Using blue as background.")
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    print_coordinates(&(10, 20));

    // let Some(x) = Some(10);

    if let x = 5 {
        println!("Hello");
    }
    match 5 {
        0..=0 => println!("btw 1 to 10"),
        2..=10 => println!("btw 1 to 10"),
        _ => println!("x"),
    }

    let Point { x, y } = Point { x: 10, y: 20 };

    println!("{}, {}", x, y);

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("Mothing");
        }
        Message::Move { x, y } => {
            println!("Move {},{}", x, y);
        }
        Message::Write(text) => {
            println!("Text {}", text);
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to rgb: {r},{g},{b}");
        }
        Message::ChangeColor(Color::Hsv(r, g, b)) => {
            println!("Change the color to  hsv: {r},{g},{b}");
        }
    }
    let ((feet, inches), Point { x, y }) = ((10, 20), Point { x: 30, y: 40 });
    println!("Feet {} Inches {} x {} y {}", feet, inches, x, y);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("can't override existing settings");
        }
        _ => setting_value = new_setting_value,
    }
    let mut x = 10;
    // _x = 20;
    // println!("{}", _x);

    let p = Point { x: 10, y: 20 };
    match p {
        Point { x, .. } => println!("x: {}", x),
    }
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("First: {} Last: {}", first, last);
        }
    }

    let x = Some(10);

    match x {
        Some(n) if true => {
            println!("x {} is even", n)
        }
        Some(n) => println!("x {} is odd", n),
        None => println!("x is not defined"),
    }

    let x = 1;
    match x {
        1 | 2 | 3 if false => println!("Ok"),
        _ => println!("Not Ok"),
    }

    let msg = SimpleMessage::Hello { id: 5 };
    match msg {
        SimpleMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found id in range: {}", id_variable),
        SimpleMessage::Hello { id: 10..=12 } => println!("Value in 10..=12 range"),
        SimpleMessage::Hello { id } => println!("Value {}", id),
    }
    let x = 10;
    match x {
        v @ 1..=10 => println!("Val {} falls in range", v),
        v => println!("Val {} is outside range", v),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Coord: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}
enum SimpleMessage {
    Hello { id: i32 },
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
