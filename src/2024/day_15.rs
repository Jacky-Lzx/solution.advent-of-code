use advent_of_code::utils::pos::Pos;
use anyhow::Result;
use std::{collections::VecDeque, fs};

#[derive(Clone, PartialEq, Copy)]
enum Object {
    Robot,
    Box,
    Wall,
    Empty,
}

#[derive(Clone, PartialEq, Copy)]
enum Object2 {
    Robot,
    BoxL,
    BoxR,
    Wall,
    Empty,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn get_available_pos_in_direction(
    robot_pos: &Pos,
    direction: &Movement,
    map: &[Vec<Object>],
) -> Option<Pos> {
    let move_vec = match direction {
        Movement::Up => (-1, 0),
        Movement::Down => (1, 0),
        Movement::Left => (0, -1),
        Movement::Right => (0, 1),
    };

    let (x_len, y_len) = (map.len() as i32, map[0].len() as i32);

    let mut cur_pos = *robot_pos;

    loop {
        let next_pos = Pos {
            x: cur_pos.x + move_vec.0,
            y: cur_pos.y + move_vec.1,
        };

        if next_pos.x < 0 || next_pos.x >= x_len || next_pos.y < 0 || next_pos.y >= y_len {
            return None;
        }

        match map[next_pos.x as usize][next_pos.y as usize] {
            Object::Wall => return None,
            Object::Empty => return Some(next_pos),
            _ => {}
        }

        cur_pos = next_pos;
    }
}

fn part_1(map: &[Vec<Object>], movements: &[Movement]) -> Result<()> {
    let mut map = map.to_vec();

    let mut robot_pos = Pos { x: 0, y: 0 };
    'outer: for (x, row) in map.iter_mut().enumerate() {
        for (y, c) in row.iter_mut().enumerate() {
            if *c == Object::Robot {
                robot_pos = Pos {
                    x: x as i32,
                    y: y as i32,
                };
                *c = Object::Empty;
                break 'outer;
            }
        }
    }

    for m in movements {
        let move_vec = match m {
            Movement::Up => (-1, 0),
            Movement::Down => (1, 0),
            Movement::Left => (0, -1),
            Movement::Right => (0, 1),
        };

        if let Some(replace_pos) = get_available_pos_in_direction(&robot_pos, m, &map) {
            let next_pos = Pos {
                x: robot_pos.x + move_vec.0,
                y: robot_pos.y + move_vec.1,
            };

            map[replace_pos.x as usize][replace_pos.y as usize] =
                map[next_pos.x as usize][next_pos.y as usize];
            map[next_pos.x as usize][next_pos.y as usize] = Object::Empty;

            robot_pos = next_pos;
        }
    }

    // print the map
    for (x, row) in map.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if robot_pos.x == x as i32 && robot_pos.y == y as i32 {
                print!("@");
            } else {
                match c {
                    Object::Robot => print!("@"),
                    Object::Box => print!("O"),
                    Object::Wall => print!("#"),
                    Object::Empty => print!("."),
                }
            }
        }
        println!();
    }

    let mut count = 0;
    for (x, row) in map.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if c == &Object::Box {
                count += x * 100 + y
            }
        }
    }

    println!("Part 1: {}", count);

    Ok(())
}

fn part_2(map: &[Vec<Object2>], movements: &[Movement]) -> Result<()> {
    let mut map = map.to_vec();

    let mut robot_pos = Pos { x: 0, y: 0 };
    'outer: for (x, row) in map.iter_mut().enumerate() {
        for (y, c) in row.iter_mut().enumerate() {
            if *c == Object2::Robot {
                robot_pos = Pos {
                    x: x as i32,
                    y: y as i32,
                };
                *c = Object2::Empty;
                break 'outer;
            }
        }
    }

    for m in movements {
        move_in_direction(&mut robot_pos, *m, &mut map);
    }

    // Print the map
    for (x, row) in map.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if robot_pos.x == x as i32 && robot_pos.y == y as i32 {
                print!("@");
            } else {
                match c {
                    Object2::Robot => print!("@"),
                    Object2::BoxL => print!("["),
                    Object2::BoxR => print!("]"),
                    Object2::Wall => print!("#"),
                    Object2::Empty => print!("."),
                }
            }
        }
        println!();
    }

    let mut count = 0;
    for (x, row) in map.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if c == &Object2::BoxL {
                count += x * 100 + y
            }
        }
    }

    println!("Part 2: {}", count);

    Ok(())
}

fn move_in_direction(robot_pos: &mut Pos, direction: Movement, map: &mut [Vec<Object2>]) {
    let move_vec = match direction {
        Movement::Up => (-1, 0),
        Movement::Down => (1, 0),
        Movement::Left => (0, -1),
        Movement::Right => (0, 1),
    };

    let mut influenced_blocks = Vec::new();

    let mut frontier = VecDeque::new();

    frontier.push_back(*robot_pos);

    let mut movable = true;
    while !frontier.is_empty() {
        let current_pos = frontier.pop_front().unwrap();

        let next_pos = Pos {
            x: current_pos.x + move_vec.0,
            y: current_pos.y + move_vec.1,
        };

        if next_pos.x < 0
            || next_pos.x >= map.len() as i32
            || next_pos.y < 0
            || next_pos.y >= map[0].len() as i32
        {
            panic!("Out of bound should never happen.");
        }

        match map[next_pos.x as usize][next_pos.y as usize] {
            Object2::BoxL => {
                if direction != Movement::Left {
                    frontier.push_back(Pos {
                        x: next_pos.x,
                        y: next_pos.y + 1,
                    });
                }
                frontier.push_back(next_pos);
            }
            Object2::BoxR => {
                if direction != Movement::Right {
                    frontier.push_back(Pos {
                        x: next_pos.x,
                        y: next_pos.y - 1,
                    });
                }
                frontier.push_back(next_pos);
            }
            Object2::Wall => {
                movable = false;
                break;
            }
            _ => {}
        }

        // println!("Influenced blocks: {:?}", influenced_blocks);
        influenced_blocks.push(current_pos);
    }

    if movable {
        influenced_blocks.sort_by(|a, b| match direction {
            Movement::Up => match a.x.cmp(&b.x) {
                std::cmp::Ordering::Equal => a.y.cmp(&b.y),
                x => x,
            },
            Movement::Down => match b.x.cmp(&a.x) {
                std::cmp::Ordering::Equal => a.y.cmp(&b.y),
                x => x,
            },
            Movement::Left => match a.y.cmp(&b.y) {
                std::cmp::Ordering::Equal => a.y.cmp(&b.y),
                x => x,
            },
            Movement::Right => match b.y.cmp(&a.y) {
                std::cmp::Ordering::Equal => a.y.cmp(&b.y),
                x => x,
            },
        });
        influenced_blocks.dedup_by(|a, b| a.x == b.x && a.y == b.y);
        // println!("Moving blocks: {:?}", influenced_blocks);

        for pos in influenced_blocks.iter() {
            let next_pos = Pos {
                x: pos.x + move_vec.0,
                y: pos.y + move_vec.1,
            };

            map[next_pos.x as usize][next_pos.y as usize] = map[pos.x as usize][pos.y as usize];
            map[pos.x as usize][pos.y as usize] = Object2::Empty;
        }

        *robot_pos = Pos {
            x: robot_pos.x + move_vec.0,
            y: robot_pos.y + move_vec.1,
        };
    }
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_15.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let empty_line_idx = lines
        .iter()
        .position(|&line| line.trim().is_empty())
        .expect("No empty line is found in the input file");

    // Build the map
    let map = lines[..empty_line_idx]
        .iter()
        .map(|&line| {
            line.chars()
                .map(|c| match c {
                    '@' => Object::Robot,
                    'O' => Object::Box,
                    '#' => Object::Wall,
                    '.' => Object::Empty,
                    _ => panic!("Unknown character in the map: {}", c),
                })
                .collect::<Vec<Object>>()
        })
        .collect::<Vec<Vec<Object>>>();

    let movements = lines[empty_line_idx + 1..]
        .iter()
        .map(|&line| {
            line.chars()
                .map(|c| match c {
                    '^' => Movement::Up,
                    'v' => Movement::Down,
                    '<' => Movement::Left,
                    '>' => Movement::Right,
                    _ => panic!("Unknown movement command: {}", c),
                })
                .collect::<Vec<Movement>>()
        })
        .collect::<Vec<Vec<Movement>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<Movement>>();

    part_1(&map, &movements)?;

    let map_2 = lines[..empty_line_idx]
        .iter()
        .map(|&line| {
            line.chars().fold(vec![], |mut vector, c| {
                match c {
                    '@' => {
                        vector.push(Object2::Robot);
                        vector.push(Object2::Empty)
                    }
                    'O' => {
                        vector.push(Object2::BoxL);
                        vector.push(Object2::BoxR);
                    }
                    '#' => {
                        vector.push(Object2::Wall);
                        vector.push(Object2::Wall);
                    }
                    '.' => {
                        vector.push(Object2::Empty);
                        vector.push(Object2::Empty);
                    }
                    _ => panic!("Unknown character in the map: {}", c),
                };
                vector
            })
        })
        .collect::<Vec<Vec<Object2>>>();

    part_2(&map_2, &movements)?;

    Ok(())
}
