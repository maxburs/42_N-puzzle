use std::fmt;

mod parse;
mod generate_final;

pub use self::parse::from_raw;

pub struct Puzzle {
    data: Vec<Vec<u32>>,
    x: usize,
    y: usize,
}

pub fn target_of_size(size: usize) -> Result<Puzzle, String> {
    parse::from_data(generate_final::generate_final(size))
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self.data)
    }
}

impl super::a_star::Expandable for Puzzle {
    fn expand(&self) -> Vec<Self> {
        let size = self.data.len();
        let mut moves = Vec::new();

        if self.x > 0 {
            moves.push((self.x - 1, self.y));
        }
        if self.x  + 1 < size {
            moves.push((self.x + 1, self.y));
        }
        if self.y > 0 {
            moves.push(((self.x, self.y - 1)));
        }
        if self.y + 1 < size {
            moves.push(((self.x, self.y + 1)));
        }
        moves.iter()
            .map(|m| self.move_space(m.0, m.1))
            .collect()
    }
}

impl Puzzle {
    fn move_space(&self, x: usize, y: usize) -> Puzzle {
        let mut data = self.data.clone();

        let swap = data[y][x];
        data[y][x] = data[self.y][self.x];
        data[self.y][self.x] = swap;

        Puzzle { data, x, y, }
    }
}


