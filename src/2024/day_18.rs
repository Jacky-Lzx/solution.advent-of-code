use advent_of_code::utils::pos::{DIRECTIONS, Pos, in_bound};
use anyhow::Result;
use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(Clone, Eq, PartialEq)]
enum Status {
    Corrupted,
    Empty,
}

struct Point {
    pos: Pos,
    point: i32,
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.point.cmp(&self.point)
    }
}

fn bfs(map: &[Vec<Status>], start_pos: Pos, end_pos: Pos) -> Result<i32> {
    let mut frontier = BinaryHeap::<Point>::new();
    let mut visited = HashSet::<Pos>::new();

    let size = map.len();

    frontier.push(Point {
        pos: start_pos,
        point: 0,
    });

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if visited.contains(&current.pos) {
            continue;
        }

        if current.pos == end_pos {
            return Ok(current.point);
        }

        visited.insert(current.pos);

        for dir in DIRECTIONS {
            let neighbor = current.pos + dir.to_pos();

            if !in_bound(&neighbor, (size, size)) {
                continue;
            }

            if Status::Corrupted == map[neighbor.y as usize][neighbor.x as usize] {
                continue;
            }

            frontier.push(Point {
                pos: neighbor,
                point: current.point + 1,
            });
        }
    }

    anyhow::bail!("No path found from {:?} to {:?}", start_pos, end_pos);
}

fn solve(bytes_pos: &[Pos], size: i32, num: i32) -> Result<i32> {
    let mut map = vec![vec![Status::Empty; size as usize]; size as usize];

    bytes_pos.iter().take(num as usize).for_each(|pos| {
        let x = pos.x as usize;
        let y = pos.y as usize;

        map[y][x] = Status::Corrupted;
    });

    bfs(
        &map,
        Pos { x: 0, y: 0 },
        Pos {
            x: size - 1,
            y: size - 1,
        },
    )
}

fn part_1(bytes_pos: &[Pos], size: i32, num: i32) -> Result<()> {
    let ans = solve(bytes_pos, size, num)?;

    println!("Part 1: {}", ans);

    Ok(())
}

fn part_2(bytes_pos: &[Pos], size: i32) -> Result<()> {
    let (mut left, mut right) = (0, bytes_pos.len() as i32); // [left, right)

    while left < right {
        let mid = (left + right) / 2;

        if solve(bytes_pos, size, mid).is_err() {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    let byte = bytes_pos.get(left as usize - 1).unwrap();

    println!("Part 2: {},{}", byte.x, byte.y);

    Ok(())
}

fn main() -> Result<()> {
    let (contents, size, num) = (fs::read_to_string("assets/2024/day_18.input")?, 71, 1024);
    // let (contents, size, num) = (fs::read_to_string("assets/test.input")?, 7, 12);
    // println!("File contents:\n{}", contents);

    let bytes_pos: Vec<Pos> = contents
        .lines()
        .map(|line| {
            let arrs = line
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Pos {
                x: arrs[0],
                y: arrs[1],
            }
        })
        .collect::<Vec<Pos>>();

    // println!("{:?}", bytes_pos);

    part_1(&bytes_pos, size, num)?;
    part_2(&bytes_pos, size)?;

    Ok(())
}
