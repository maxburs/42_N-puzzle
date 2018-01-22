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
    println!("Number of moves required: {}", solution.number_of_moves_required);
}
