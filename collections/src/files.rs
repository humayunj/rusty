use std::{
    fs::{self, File},
    io::ErrorKind,
    io::{self, Read},
};

pub fn read_file() {
    let result = File::open("hello.txt");

    let file = match File::open("hello.txt") {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => File::create("hello.txt").expect("failed to create file"),
            err => panic!("Failed to open file {}", err),
        },
    };
}

pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file_alt() -> Result<String, io::Error> {
    let username_file = File::open("hello.txt")?;

    let mut username = String::new();

    
}
