use puzzle::Puzzle;

// todo: check if puzzle has a valid combination of numbers

pub fn parse_puzzle(raw: &str) -> Result<Puzzle, String> {
    let mut lines = raw.lines()
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
        |l: &str| l.split_whitespace()
            .map(|n| n.parse::<i32>().expect("Failed to parse puzzle value"))
            .collect()
    ).collect();

    // validate puzzle size
    if puzzle.len() != size {
        return Err(format!("Expected {} rows, got {}", size, puzzle.len()));
    }
    for (i, row) in puzzle.iter().enumerate() {
        if row.len() != size {
            return Err(format!("Row {}: expected {} numbers, got {}", i, row.len(), size));
        }
    };

    Ok(Puzzle {
        size: size,
        content: puzzle,
    })
}
