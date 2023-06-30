use catty::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::build(&args) {
        Err(e) => {
            eprintln!("Parse Error: {}", e);
            process::exit(1);
        }
        Ok(config) => config,
    };

    if let Err(e) = catty::run(&config) {
        eprintln!("Program Error: {}", e.to_string());
        process::exit(1);
    }
}
