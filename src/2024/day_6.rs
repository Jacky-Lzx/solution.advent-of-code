use std::{collections::HashSet, fs};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[derive(PartialEq, Clone)]
enum Status {
    Obstacle,
    Visited,
    Unvisited,
}

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Obstacle => write!(f, "#"),
            Status::Visited => write!(f, "^"),
            Status::Unvisited => write!(f, "."),
        }
    }
}

fn part_1(map: &[Vec<Status>]) {
    let mut map = map.to_vec();

    let mut point = Point { x: 0, y: 0 };

    let (x_len, y_len) = (map.len() as i32, map[0].len() as i32);

    'outer: for (i, row) in map.iter().enumerate() {
        for (j, status) in row.iter().enumerate() {
            if *status == Status::Visited {
                point = Point {
                    x: i as i32,
                    y: j as i32,
                };
                break 'outer;
            }
        }
    }

    assert!(
        point.x != 0 && point.y != 0,
        "No starting point found in the map"
    );

    let mut cur_direction_index = 0;

    let mut pos = point;
    'outer: loop {
        loop {
            pos += DIRECTIONS[cur_direction_index].0;

            if !in_bounds(pos, x_len, y_len) {
                break 'outer;
            }

            let status = &mut map[pos.x as usize][pos.y as usize];
            if *status == Status::Obstacle {
                pos -= DIRECTIONS[cur_direction_index].0;
                break;
            }
            *status = Status::Visited;
        }

        cur_direction_index = (cur_direction_index + 1) % DIRECTIONS.len();
    }

    // map.iter().for_each(|row| {
    //     println!(
    //         "{}",
    //         row.iter()
    //             .map(|status| format!("{:?}", status))
    //             .collect::<Vec<String>>()
    //             .join("")
    //     );
    // });

    let count = map
        .iter()
        .flatten()
        .filter(|&status| *status == Status::Visited)
        .count();

    println!("Part 1: {}", count);
}

fn in_bounds(pos: Point, x_len: i32, y_len: i32) -> bool {
    pos.x >= 0 && pos.x < x_len && pos.y >= 0 && pos.y < y_len
}

fn check_loop(mut cur_map: Vec<Vec<Status>>, start_pos: Point) -> bool {
    let (x_len, y_len) = (cur_map.len() as i32, cur_map[0].len() as i32);

    let mut cur_pos = start_pos;
    let mut cur_dir_index = 0;

    let mut cur_dir_map: Vec<Vec<HashSet<Direct>>> = cur_map
        .clone()
        .iter()
        .map(|line| line.iter().map(|_| HashSet::new()).collect())
        .collect();
    cur_dir_map[start_pos.x as usize][start_pos.y as usize].insert(Direct::Up);

    loop {
        cur_pos += DIRECTIONS[cur_dir_index].0;

        if !in_bounds(cur_pos, x_len, y_len) {
            return false;
        }

        if cur_map[cur_pos.x as usize][cur_pos.y as usize] == Status::Obstacle {
            cur_pos -= DIRECTIONS[cur_dir_index].0;
            cur_dir_index = (cur_dir_index + 1) % DIRECTIONS.len();
            cur_dir_map[cur_pos.x as usize][cur_pos.y as usize].insert(DIRECTIONS[cur_dir_index].1);
            continue;
        }

        if cur_map[cur_pos.x as usize][cur_pos.y as usize] == Status::Visited
            && cur_dir_map[cur_pos.x as usize][cur_pos.y as usize]
                .contains(&DIRECTIONS[cur_dir_index].1)
        {
            return true;
        }

        cur_map[cur_pos.x as usize][cur_pos.y as usize] = Status::Visited;
        cur_dir_map[cur_pos.x as usize][cur_pos.y as usize].insert(DIRECTIONS[cur_dir_index].1);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direct {
    Left,
    Right,
    Up,
    Down,
}
const DIRECTIONS: [(Point, Direct); 4] = [
    (Point { x: -1, y: 0 }, Direct::Up),   // up
    (Point { x: 0, y: 1 }, Direct::Right), // right
    (Point { x: 1, y: 0 }, Direct::Down),  // down
    (Point { x: 0, y: -1 }, Direct::Left), // left
];

fn part_2(orig_map: &[Vec<Status>]) {
    let mut map = orig_map.to_vec();

    let mut start_pos = Point { x: 0, y: 0 };
    'outer: for (i, row) in map.iter().enumerate() {
        for (j, status) in row.iter().enumerate() {
            if *status == Status::Visited {
                start_pos = Point {
                    x: i as i32,
                    y: j as i32,
                };
                break 'outer;
            }
        }
    }
    assert!(
        start_pos.x != 0 && start_pos.y != 0,
        "No starting point found in the map"
    );

    let mut pos = start_pos;

    let mut direct_map: Vec<Vec<HashSet<Direct>>> = map
        .clone()
        .iter()
        .map(|line| line.iter().map(|_| HashSet::new()).collect())
        .collect();
    direct_map[start_pos.x as usize][start_pos.y as usize].insert(Direct::Up);

    let mut cur_direction_index = 0;

    let (x_len, y_len) = (map.len() as i32, map[0].len() as i32);

    let mut positions: HashSet<Point> = HashSet::new();

    loop {
        // Assume an obstacle is placed in the next position, check if a loop is formed
        let next_pos = pos + DIRECTIONS[cur_direction_index].0;
        if in_bounds(next_pos, x_len, y_len)
            && map[next_pos.x as usize][next_pos.y as usize] != Status::Obstacle
        {
            // NOTE: Should start from the original map.
            // The reason is that placing an obstacle in the next position may block the path to the current position.
            let mut temp_map = map.clone();
            temp_map[next_pos.x as usize][next_pos.y as usize] = Status::Obstacle;
            if check_loop(temp_map, start_pos) {
                positions.insert(next_pos);
            }
        }

        pos += DIRECTIONS[cur_direction_index].0;

        if !in_bounds(pos, x_len, y_len) {
            break;
        }

        if map[pos.x as usize][pos.y as usize] == Status::Obstacle {
            pos -= DIRECTIONS[cur_direction_index].0;
            cur_direction_index = (cur_direction_index + 1) % DIRECTIONS.len();
            direct_map[pos.x as usize][pos.y as usize].insert(DIRECTIONS[cur_direction_index].1);
            continue;
        }

        map[pos.x as usize][pos.y as usize] = Status::Visited;
        direct_map[pos.x as usize][pos.y as usize].insert(DIRECTIONS[cur_direction_index].1);
    }

    // map.iter().for_each(|row| {
    //     println!(
    //         "{}",
    //         row.iter()
    //             .map(|status| format!("{:?}", status))
    //             .collect::<Vec<String>>()
    //             .join("")
    //     );
    // });

    assert!(!positions.contains(&start_pos));

    println!("Part 2: {}", positions.len());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("assets/2024/day_6.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let map: Vec<Vec<Status>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Status::Obstacle,
                    '.' => Status::Unvisited,
                    '^' => Status::Visited,
                    c => panic!("Unexpected character in input: {}", c),
                })
                .collect()
        })
        .collect();

    part_1(&map);
    part_2(&map);

    Ok(())
}
