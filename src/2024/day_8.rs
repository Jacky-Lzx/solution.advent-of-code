use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl std::ops::Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn in_bounds(pos: &Pos, x_len: usize, y_len: usize) -> bool {
    pos.x >= 0 && (pos.x as usize) < x_len && pos.y >= 0 && (pos.y as usize) < y_len
}

fn part_1(map: &[Vec<char>]) {
    let (x_len, y_len) = (map.len(), map[0].len());

    // Get the location of each antenna
    let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| match c {
            '.' => (),
            &antenna => {
                antennas.entry(antenna).or_default().push(Pos {
                    x: i as i32,
                    y: j as i32,
                });
            }
        });
    });

    let mut antinode_map: HashSet<Pos> = HashSet::new();

    antennas.iter().for_each(|(_antenna, positions)| {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let left = positions[i] - positions[j] + positions[i];
                if in_bounds(&left, x_len, y_len) {
                    antinode_map.insert(left);
                }

                let right = positions[j] - positions[i] + positions[j];
                if in_bounds(&right, x_len, y_len) {
                    antinode_map.insert(right);
                }
            }
        }
    });

    println!("Part 1: {}", antinode_map.len());
}

fn part_2(map: &[Vec<char>]) {
    let (x_len, y_len) = (map.len(), map[0].len());

    // Get the location of each antenna
    let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| match c {
            '.' => (),
            &antenna => {
                antennas.entry(antenna).or_default().push(Pos {
                    x: i as i32,
                    y: j as i32,
                });
            }
        });
    });

    let mut antinode_map: HashSet<Pos> = HashSet::new();

    antennas.iter().for_each(|(_antenna, positions)| {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let mut vec_1 = positions[i] - positions[j];

                for divider in (2..=vec_1.x.abs().max(vec_1.y.abs())).rev() {
                    while vec_1.x % divider == 0 && vec_1.y % divider == 0 {
                        vec_1 = Pos {
                            x: vec_1.x / divider,
                            y: vec_1.y / divider,
                        };
                    }
                }

                let mut pos = positions[i] + vec_1;
                while in_bounds(&pos, x_len, y_len) {
                    antinode_map.insert(pos);
                    pos = vec_1 + pos;
                }

                let mut pos = positions[j] - vec_1;
                while in_bounds(&pos, x_len, y_len) {
                    antinode_map.insert(pos);
                    pos = pos - vec_1;
                }
            }
        }
    });

    antennas.iter().for_each(|(_antenna, positions)| {
        positions.iter().for_each(|&pos| {
            antinode_map.insert(pos);
        });
    });

    println!("Part 2: {}", antinode_map.len());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("assets/2024/day_8.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    part_1(&map);
    part_2(&map);

    Ok(())
}
