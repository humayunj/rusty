use std::{error::Error, fmt::format, fs, io};
pub struct Config {
    pub file: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough args");
        }
        let file = args[1].clone();
        return Ok(Config { file });
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = match fs::read_to_string(config.file.clone()) {
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                return Err(format(format_args!("file '{}' doesn't exists", config.file)).into())
            }
            _ => return Err("something went wrong".into()),
        },
        Ok(c) => c,
    };
    println!("{}", content);
    return Ok(());
}
