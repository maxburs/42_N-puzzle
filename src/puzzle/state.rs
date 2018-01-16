use std::cmp::Eq;
use std::cmp::PartialEq;

// todo: implement hash so it ignores x & y

#[derive(Clone, Hash)]
pub struct State {
    pub data: Vec<Vec<i32>>,
    x: usize,
    y: usize,
}

pub fn new(data: Vec<Vec<i32>>) -> State {
    let mut coordinate: Option<(usize, usize)> = None;

    for (y, row) in data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == 0 {
                coordinate = Some((x, y));
            }
        }
    }

    if let Some((x, y)) = coordinate {
        return State { data, x, y };
    }

    panic!("Failed to find empty space in puzzle");
}

impl State {
    fn expand(&self) -> Vec<State> {
        let size = self.data.len();

        // todo: numbers wont go below zero and get culled

        [
            (self.x - 1, self.y),
            (self.x + 1, self.y),
            (self.x, self.y - 1),
            (self.x, self.y + 1),
        ].iter()
            .filter(|&m| (m.0 >= 0 && m.0 < size && m.1 >= 0 && m.1 < size))
            .map(|m| self.swap(m.0, m.1))
            .collect()
    }
    fn swap(&self, x: usize, y: usize) -> State {
        let mut data = self.data.clone();

        let swap = data[y][x];
        data[y][x] = data[self.y][self.x];
        data[self.y][self.x] = swap;

        State {
            data: data,
            x: x,
            y: y,
        }
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.data == other.data
    }
}

#[test]
fn test_expand() {
    assert_eq!(
        [
            State {
                data: [[0, 1], [1, 1],],
                x: 0,
                y: 0,
            },
            State {
                data: [[1, 1], [1, 0],],
                x: 1,
                y: 1,
            }
        ],
        State {
            data: [[1, 0], [1, 1],],
            x: 1,
            y: 0,
        }.expand()
    );
}
