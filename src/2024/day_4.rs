use std::fs;

fn part_1(arr: &[Vec<char>]) {
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
    const CHARS: [char; 3] = ['M', 'A', 'S'];

    let mut count = 0;

    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            let c = arr[i][j];
            if c == 'X' {
                for dir in DIRECTIONS {
                    let mut is_valid = true;
                    for index in 1..=CHARS.len() {
                        let point = (
                            i as i32 + dir.0 * index as i32,
                            j as i32 + dir.1 * index as i32,
                        );
                        if point.0 < 0
                            || point.0 >= arr.len() as i32
                            || point.1 < 0
                            || point.1 >= arr[i].len() as i32
                        {
                            is_valid = false;
                            break;
                        }

                        if arr[point.0 as usize][point.1 as usize] != CHARS[index - 1] {
                            is_valid = false;
                            break;
                        }
                    }
                    if is_valid {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Part 1: {}", count);
}

fn part_2(arr: &[Vec<char>]) {
    const DIRECTIONS: [(i32, i32); 2] = [(-1, 1), (1, 1)];
    const CHARS: [char; 2] = ['M', 'S'];

    let mut count = 0;

    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            let c = arr[i][j];
            if c == 'A' {
                let mut is_valid = true;
                for dir in DIRECTIONS {
                    let mut is_valid_one_dir = false;
                    for index in 1..=CHARS.len() {
                        let point = (i as i32 + dir.0, j as i32 + dir.1);
                        if point.0 < 0
                            || point.0 >= arr.len() as i32
                            || point.1 < 0
                            || point.1 >= arr[i].len() as i32
                        {
                            break;
                        }

                        if arr[point.0 as usize][point.1 as usize] != CHARS[index - 1] {
                            continue;
                        }

                        // Check the mirrored point
                        let point_mirror = (i as i32 - dir.0, j as i32 - dir.1);
                        if point_mirror.0 < 0
                            || point_mirror.0 >= arr.len() as i32
                            || point_mirror.1 < 0
                            || point_mirror.1 >= arr[i].len() as i32
                        {
                            break;
                        }

                        if arr[point_mirror.0 as usize][point_mirror.1 as usize]
                            != CHARS[CHARS.len() - index]
                        {
                            continue;
                        }

                        is_valid_one_dir = true;
                        break;
                    }
                    if !is_valid_one_dir {
                        is_valid = false;
                    }
                }
                if is_valid {
                    count += 1;
                }
            }
        }
    }

    println!("Part 2: {}", count);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("assets/2024/day_4.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    part_1(&lines);
    part_2(&lines);

    Ok(())
}
