#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    NYC,
    Texas,
}
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}
impl Coin {
    fn value(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State is {:?}", state);
                25
            }
        }
    }
}

fn main() {
    let scale = 2;
    let r = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("Area is {}", r.area());
    println!("Rect is {:#?}", r);
    println!(
        "Can hold: {}",
        r.can_hold(&Rectangle {
            width: 10,
            height: 20
        })
    );

    dbg!(&r);

    let ip = IpAddr::V4(10, 10, 10, 10);

    let some_numer = Some(10);
    let absent: Option<i32> = None;

    let coin = Coin::Quarter(UsState::Texas);
    coin.value();

    let mut i = Some(10);
    i = plus_one(i);
    println!("Val: {:?}", i);
    i = None;
    plus_one(i);
    println!("Val: {:?}", i);

    let x: Option<i8> = None;

    if let None = x {
        println!("x is none")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
