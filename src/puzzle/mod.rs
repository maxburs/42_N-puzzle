use std::fmt;

mod parse;

pub struct Puzzle {
    size: usize,
    content: Vec<Vec<i32>>,
}
   
pub fn new(raw: &str) -> Result<Puzzle, String> {
    parse::parse_puzzle(raw)
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self.content)
    }
}
