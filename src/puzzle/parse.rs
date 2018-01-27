use puzzle::Puzzle;

// todo: check if puzzle has a valid combination of numbers

fn validate(puzzle: &Vec<Vec<u32>>, size: usize) -> Result<(), String> {
    let mut found = vec![false; size * size];

    // Validate puzzle height.
    if puzzle.len() != size {
        return Err(format!("Expected {} rows, got {}", size, puzzle.len()));
    }

    for (y, row) in puzzle.iter().enumerate() {
        // Validate puzzle width.
        if row.len() != size {
            return Err(format!(
                "Row {}: expected {} numbers, got {}",
                y,
                row.len(),
                size
            ));
        }

        // Mark needed values off as we find them.
        for value in row {
            found[*value as usize] = true;
        }
    }

    // Make sure all values were found.
    for (i, value) in found.iter().enumerate() {
        if *value == false {
            return Err(format!("Could not find {} in puzzle", i));
        }
    }

    Ok(())
}

fn find_center(data: &Vec<Vec<u32>>) -> Result<(usize, usize), String> {

    let mut coordinate: Option<(usize, usize)> = None;

    for (y, row) in data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == 0 {
                coordinate = Some((x, y));
            }
        }
    }

    match coordinate {
        Some(c) => Ok(c),
        None => Err(String::from("Failed to find empty space in puzzle"))
    }
}

pub fn from_raw(raw: &str) -> Result<Puzzle, String> {
    let mut lines = raw.lines()
        .filter(|&l| l.starts_with("#") == false)
        .map(|l| match l.find("#") {
            Some(i) => &l[..i],
            None => l,
        });

    let size: usize = lines
        .next()
        .expect("First line missing in puzzle")
        .trim()
        .parse()
        .expect("Failed to parse puzzle size");

    let data: Vec<Vec<u32>> = lines
        .map(|l: &str| {
            l.split_whitespace()
                .map(|n| n
                    .parse::<u32>()
                    .expect("Failed to parse puzzle value")
                )
                .collect()
        })
        .collect();
    
    return from_data(data);
}

pub fn from_data(data: Vec<Vec<u32>>) -> Result<Puzzle, String> {
    validate(&data, data.len())?;

    let (x, y) = find_center(&data)?;

    Ok(Puzzle { x, y, data, })
}
