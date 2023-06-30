use std::{thread, time::Duration};

mod channels;
mod mutex;
fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("vec: {:?}", v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    // channels::main();
    mutex::main();
}
