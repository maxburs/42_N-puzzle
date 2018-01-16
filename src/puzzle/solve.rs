use puzzle::Puzzle;
use puzzle::Solution;
use puzzle::state;
use std::collections::HashMap;
use std::rc::Rc;

fn generate_final_state(size: usize) -> state::State {
    let mut left = 0;
    let mut top = 0;
    let mut right = size - 1;
    let mut bottom = size - 1;

    let mut puz = vec![vec![0; size]; size];

    let mut counter = 1;

    loop {
        for x in left..(right + 1) {
            puz[top][x] = counter;
            counter += 1;
        }
        if top == bottom { break; }
        top += 1;
        for y in top..(bottom + 1) {
            puz[y][right] = counter;
            counter += 1;
        }
        if right == left { break; }
        right -= 1;
        for x in (left..(right + 1)).rev() {
            puz[bottom][x] = counter;
            counter += 1;
        }
        if top == bottom { break; }
        bottom -= 1;
        for y in (top..(bottom + 1)).rev() {
            puz[y][left] = counter;
            counter += 1;
        }
        if right == left { break; }
        left += 1;
    }

    puz[top][left] = 0;

    state::new(puz)
}

#[test]
fn validate_puzzle_gen() {
    assert_eq!(
        generate_final_state(3).data,
        [
            [1, 2, 3],
            [8, 0, 4],
            [7, 6, 5]
        ]
    );
    assert_eq!(
        generate_final_state(4).data,
        [
            [ 1,  2,  3,  4],
            [12, 13, 14,  5],
            [11,  0, 15,  6],
            [10,  9,  8,  7]
        ]
    );
    assert_eq!(
        generate_final_state(5).data,
        [
            [ 1,  2,  3,  4, 5],
            [16, 17, 18, 19, 6],
            [15, 24,  0, 20, 7],
            [14, 23, 22, 21, 8],
            [13, 12, 11, 10, 9]
        ]
    );
}

pub fn solve(puzzle: &Puzzle) -> Option<Solution> {
    // todo: use unsafe code instead of reference counting
    let start = Rc::new(puzzle.state.clone());

    // open_rank stores the open states sorted by ranking
    let mut open_rank = Vec::new();
    let mut states = HashMap::new();
    let final_state = generate_final_state(puzzle.size);

    open_rank.push(Rc::clone(&start));
    states.insert(Rc::clone(&start), true);

    loop {
        let e = if let Some(state) = open_rank.pop() {
            state
        } else {
            return None;
        };

        if *e == final_state {
            return Some(Solution {
                complexity_time: 0,
                complexity_space: 0,
                moves: ()
            });
        };

    }
}
