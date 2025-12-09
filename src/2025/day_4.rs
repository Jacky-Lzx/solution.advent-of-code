const EMPTY: char = '.';
const PAPER: char = '@';
const UNINITIALIZED: char = 'a';

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn remove(map: &mut [Vec<char>], counts: &mut [Vec<i32>]) -> i32 {
    let row_num = counts.len();
    let col_num = counts[0].len();

    let mut results = 0;
    for row_i in 0..row_num {
        for col_i in 0..col_num {
            if map[row_i][col_i] == PAPER && counts[row_i][col_i] < 4 {
                results += 1;

                counts[row_i][col_i] = 0;
                map[row_i][col_i] = EMPTY;

                for dir in &DIRECTIONS {
                    let dst_row = row_i as i32 + dir.0;
                    if dst_row < 0 || dst_row >= row_num as i32 {
                        continue;
                    }
                    let dst_col = col_i as i32 + dir.1;
                    if dst_col < 0 || dst_col >= col_num as i32 {
                        continue;
                    }

                    counts[dst_row as usize][dst_col as usize] -= 1;
                }
            }
        }
    }

    results
}

fn part_1(contents: &Vec<&str>) {
    let row_num = contents.len();

    assert!(row_num > 0);
    let col_num = contents[0].len();

    let mut counts = vec![vec![0; col_num]; row_num];

    for (row_i, row) in contents.iter().enumerate() {
        for (col_i, ch) in row.chars().enumerate() {
            if ch == EMPTY {
                continue;
            }

            for dir in &DIRECTIONS {
                let dst_row = row_i as i32 + dir.0;
                if dst_row < 0 || dst_row >= row_num as i32 {
                    continue;
                }
                let dst_col = col_i as i32 + dir.1;
                if dst_col < 0 || dst_col >= col_num as i32 {
                    continue;
                }

                counts[dst_row as usize][dst_col as usize] += 1;
            }
        }
    }

    let mut result = 0;
    for (row_i, row) in contents.iter().enumerate() {
        for (col_i, ch) in row.chars().enumerate() {
            if ch == PAPER && counts[row_i][col_i] < 4 {
                result += 1;
            }
        }
    }

    println!("Part 1: {}", result);
}

fn part_2(contents: &Vec<&str>) {
    let row_num = contents.len();

    assert!(row_num > 0);
    let col_num = contents[0].len();

    let mut counts = vec![vec![0; col_num]; row_num];
    let mut map = vec![vec![UNINITIALIZED; col_num]; row_num];

    for (row_i, row) in contents.iter().enumerate() {
        for (col_i, ch) in row.chars().enumerate() {
            map[row_i][col_i] = ch;

            if ch == EMPTY {
                continue;
            }

            for dir in &DIRECTIONS {
                let dst_row = row_i as i32 + dir.0;
                if dst_row < 0 || dst_row >= row_num as i32 {
                    continue;
                }
                let dst_col = col_i as i32 + dir.1;
                if dst_col < 0 || dst_col >= col_num as i32 {
                    continue;
                }

                counts[dst_row as usize][dst_col as usize] += 1;
            }
        }
    }

    let mut result = 0;
    let mut increase = remove(&mut map, &mut counts);

    while increase > 0 {
        println!("Removed {} papers", increase);
        result += increase;
        increase = remove(&mut map, &mut counts);
    }

    println!("Part 2: {}", result);
}
fn main() {
    let contents = std::fs::read_to_string("assets/2025/day_4.input").unwrap();
    // let contents = std::fs::read_to_string("assets/2025/test.input").unwrap();

    let contents = contents.lines().collect::<Vec<&str>>();

    part_1(&contents);
    part_2(&contents);
}
