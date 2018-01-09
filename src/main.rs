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
    let file = match get_puzzle() {
        Some(file) => file,
        None => return
    };

    let mut lines = file.lines()
        .filter(|&l| l.starts_with("#") == false)
        .map(|l|
            match l.find("#") {
                Some(i) => &l[..i],
                None => l
            }
        );

    let size: usize = lines.next()
        .expect("First line missing in puzzle")
        .trim()
        .parse()
        .expect("Failed to parse puzzle size");

    let puzzle: Vec<Vec<i32>> = lines.map(
        |l| {
            let line: Vec<i32> = l.split_whitespace()
            .map(|n| n.parse().expect("Failed to parse puzzle value"))
            .collect();
            assert_eq!(line.len(), size);
            line
        }
    ).collect();

    assert_eq!(puzzle.len(), size);

    println!("size: {}", size);

    println!("lines: {:?}", puzzle);
}
