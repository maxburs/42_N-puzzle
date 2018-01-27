pub fn generate_final(size: usize) -> Vec<Vec<u32>>{
    let mut left = 0;
    let mut top = 0;
    let mut right = size - 1;
    let mut bottom = size - 1;

    let mut data = vec![vec![0; size]; size];

    let mut counter = 1;

    loop {
        for x in left..(right + 1) {
            data[top][x] = counter;
            counter += 1;
        }
        if top == bottom { break; }
        top += 1;
        for y in top..(bottom + 1) {
            data[y][right] = counter;
            counter += 1;
        }
        if right == left { break; }
        right -= 1;
        for x in (left..(right + 1)).rev() {
            data[bottom][x] = counter;
            counter += 1;
        }
        if top == bottom { break; }
        bottom -= 1;
        for y in (top..(bottom + 1)).rev() {
            data[y][left] = counter;
            counter += 1;
        }
        if right == left { break; }
        left += 1;
    }

    data[top][left] = 0;

    data
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
