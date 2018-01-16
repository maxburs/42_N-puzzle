use std::fmt;

mod parse;
mod solve;
mod state;

pub struct Puzzle {
    size: usize,
    data: Vec<Vec<i32>>,
}

pub struct Solution {
    // Total number of states ever selected in the "opened" set (complexity in time).
    pub complexity_time: usize,
    // Maximum number of states ever represented in memory at the same time
    // during the search (complexity in size)
    pub complexity_space: usize,
    // Number of moves required to transition from the initial state to the final state,
    // according to the search.
    pub moves: (), // unimplemented
}
   
pub fn new(raw: &str) -> Result<Puzzle, String> {
    parse::parse_puzzle(raw)
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self.data)
    }
}

impl Puzzle {
    pub fn solve(&self) -> Option<Solution> {
        solve::solve(&self)
    }
}


