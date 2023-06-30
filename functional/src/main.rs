use std::thread;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Blue,
    Red,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }
        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}
fn test(f: &dyn Fn()) {
    f()
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "This user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "This user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    // let ex = |x| x;

    // println!("{}", ex(String::from("hello")));
    // println!("{}", ex(10));

    let mut list = vec![1, 2, 3];

    let mut closure = || list.push(4);
    closure();
    println!("Vector: {:?}", list);

    thread::spawn(move || println!("From thread {:?}", list))
        .join()
        .unwrap();

    test(&|| println!("Hello"));

    let mut list = ["helloo", "world"];
    println!("Before sort {:?}", list);
    list.sort_by_key(|x| x.len());
    println!("After sort {:?}", list);
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);
    let mut i = v.iter_mut();
    *(i.next().unwrap()) = 10;
    println!("{:?}", v);
    let v2 = v.iter();
    let s: i32 = v2.collec();
}
