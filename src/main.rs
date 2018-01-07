use std::env;
use std::fs::File;
use std::io::prelude::*;

// type State = [[i32]];


fn get_puzzle() -> Option<String> {
    let file_name = match env::args().nth(1) {
        Some(file_name) => file_name,
        None => {
            println!("Please provide file name as program argument.");
            return None;
        }
    };
    let mut file_contents = String::new();
    match File::open(& file_name) {
        Ok(mut file) => file.read_to_string(&mut file_contents).unwrap(),
        Err(_) => {
            println!("Failed to open {}", file_name);
            return None;
        }
    };
    Some(file_contents)
}

fn main() {
    match get_puzzle() {
        Some(file) => println!("file: {}", file),
        None => return
    }
}
