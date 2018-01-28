use std::marker;
use std::fmt;

mod state;
mod solve;

pub use self::solve::solve;

pub trait Expandable: marker::Sized {
    fn expand(&self) -> Vec<Self>;
}

pub struct Solution<T> {
    // Total number of states ever selected in the "opened" set (complexity in time).
    pub complexity_time: usize,
    // Maximum number of states ever represented in memory at the same time
    // during the search (complexity in size)
    pub complexity_space: usize,
    // Number of moves required to transition from the initial state to the final state,
    // according to the search.
    pub sequence_of_states: Vec<T>, // unimplemented
    pub number_of_moves_required: usize,
}

impl<T> fmt::Display for Solution<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Time Complexity: {}\n", self.complexity_time)?;
        write!(f, "Space Complexity: {}\n", self.complexity_space)?;
        write!(f, "Solution Moves: {}\n", self.number_of_moves_required)?;
        Ok(())
    }
}
