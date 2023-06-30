use std::io;

fn main() {
    // temprature();
    // fibonacci();
    poem();
}

fn temprature() {
    let mut input = String::new();
    let opr: u8;
    println!("Enter operation:");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    io::stdin().read_line(&mut input).expect("Failed");

    opr = match input.trim().parse() {
        Ok(val) => val,
        Err(e) => {
            println!("Error: {}", e.to_string());
            0
        }
    };
    println!("{opr}");
    if opr == 1 {
        println!("30F to C => {}", fahrenheit_to_celsius(30.0))
    } else if opr == 2 {
        println!("30C to F => {}", celsius_to_fahrenheit(30.0))
    } else {
        println!("Invalid choice")
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    let f = f - 32.0;
    f * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) * 32.0
}

fn fibonacci() {
    println!("Enter n: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");

    let n: u64;
    n = match input.trim().parse() {
        Ok(v) => v,
        Err(_) => 0,
    };
    println!("fib(n) => {}", fib(n));
}

fn fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}

fn poem() {
    let day_names: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelveth",
    ];
    let a = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five gold rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtledoves",
        "And a partridge in a pear tree",
    ];

    for days in 0..a.len() {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            day_names[days]
        );
        if days == 0 {
            println!("A partridge in a pear tree");
            println!();
            continue;
        }
        for i in a.len() - days - 1..a.len() {
            println!("{}", a[i]);
        }
        println!()
    }
    println!("{}", a[a.len() - 1]);
}
