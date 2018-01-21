use puzzle::Puzzle;
use puzzle::Solution;
use puzzle::state;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

fn generate_final_data(size: usize) -> Vec<Vec<i32>> {
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

    puz
}

#[test]
fn validate_puzzle_gen() {
    assert_eq!(
        generate_final_data(3),
        [
            [1, 2, 3],
            [8, 0, 4],
            [7, 6, 5]
        ]
    );
    assert_eq!(
        generate_final_data(4),
        [
            [ 1,  2,  3,  4],
            [12, 13, 14,  5],
            [11,  0, 15,  6],
            [10,  9,  8,  7]
        ]
    );
    assert_eq!(
        generate_final_data(5),
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
    let start = Rc::new(RefCell::new(state::new(puzzle.data.clone(), 0)));

    // open_rank stores the open states sorted by ranking
    let mut open_rank = Vec::new();
    let mut states = HashMap::new();
    let final_state = generate_final_data(puzzle.size);

    println!("target state: {:?}", final_state);

    let mut complexity_time = 0;

    open_rank.push(Rc::clone(&start));
    // todo: Duplicating data here. Need to remove this somehow.

    let key = start.borrow().data.clone();
    states.insert(key, start);

    loop {
        complexity_time += 1;

        let e_cell = if let Some(state) = open_rank.pop() {
            state
        } else {
            return None;
        };

        let e = e_cell.borrow();

        if e.data == final_state {
            println!("finished!");

            for state in states.values() {
                if state.borrow().data != e.data {
                    state.borrow_mut().predecessor = None;
                }
            }

            return Some(Solution {
                complexity_time,
                complexity_space: states.len(),
                moves: ()
            });
        };

        for mut s in e.expand() {
            // Two if statements so I can mutate states in else.
            if ( if let Some(mut s_cell) = states.get(&s.data) {
                let mut s = s_cell.borrow_mut();
                if s.distance > e.distance + 1 {
                    s.distance = e.distance + 1;
                    s.predecessor = Some(Rc::clone(&e_cell));
                    if s.open == false {
                        s.open = true;
                        open_rank.push(Rc::clone(&s_cell));
                    }
                }
                true
            } else { false }) == false {
                // print!("new state:\n{}", s);

                s.predecessor = Some(Rc::clone(&e_cell));
                let s = Rc::new(RefCell::new(s));
                open_rank.push(Rc::clone(&s));
                let key = s.borrow().data.clone();
                states.insert(key, s);
            }
        }
        // println!("Remaining states: {}", states.len());
    }
}
