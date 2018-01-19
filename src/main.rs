use std::env;
use std::fs::File;
use std::io::prelude::*;

mod puzzle;

fn read_file() -> Result<String, String> {
    let file_name = match env::args().nth(1) {
        Some(file_name) => file_name,
        None => return Err(String::from("Please provide file name as program argument."))
    };
    let mut file_contents = String::new();
    match File::open(& file_name) {
        Ok(mut file) => file.read_to_string(&mut file_contents).unwrap(),
        Err(_) => return Err(format!("Failed to open {}", file_name))
    };
    Ok(file_contents)
}

#[test]
fn overflow() {
    let mut raw = String::new();
    File::open("./resources/puzzles/example_1.txt").expect("test").read_to_string(& mut raw).expect("test");

    let puzzle = puzzle::new(&raw).expect("test");

    let solution = puzzle.solve().expect("test");
}

fn main() {
    let raw = match read_file() {
        Ok(file) => file,
        Err(e) => {
            println!("{}",e);
            return;
        }
    };

    let puzzle = match puzzle::new(&raw) {
        Ok(file) => file,
        Err(e) => {
            println!("{}",e);
            return;
        }
    };

    println!("lines: {}", puzzle);

    let solution = if let Some(solution) = puzzle.solve() {
        solution
    } else {
        println!("No solution found");
        return;
    };
    
    println!("Space complexity: {}", solution.complexity_space);
    println!("Time Complexity:  {}", solution.complexity_time);
}
