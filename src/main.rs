use std::env;
use std::fs::File;
use std::io::prelude::*;

mod puzzle;
mod a_star;

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

    let puz = match puzzle::from_raw(&raw) {
        Ok(file) => file,
        Err(e) => {
            println!("{}",e);
            return;
        }
    };

    println!("lines:\n{}", puz);

    let solution = if let Some(solution) = a_star::solve(&puz, &puzzle::target_of(&puz)) {
        solution
    } else {
        println!("No solution found");
        return;
    };
    
    println!("{}", solution);
}
