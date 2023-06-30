fn main() {
    let y = {
        let mut x = 9;
        x = increament(x);
        x * 10
    };
    println!("{}", y);

    let y = if y % 2 == 0 { true } else { false };
    println!("Yo {y}");
    let x = loop {
        break 10;
    };
    println!("{x}");

    'outer: loop {
        loop {
            printRangeRev(0, 4);
            break 'outer;
        }
    }
}

fn increament(x: u32) -> u32 {
    if x % 2 == 0 {
        x + 1
    } else {
        x - 1
    }
}

fn printRangeRev(start: u32, end: u32) {
    for i in (start..end).rev() {
        println!("{i}");
    }
}
