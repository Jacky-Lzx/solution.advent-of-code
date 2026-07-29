use advent_of_code::utils::pos::{DIRECTIONS, Pos, in_bound};
use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum Element {
    Start,
    End,
    Wall,
    Empty,
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Element::Start => 'S',
            Element::End => 'E',
            Element::Wall => '#',
            Element::Empty => '.',
        };
        write!(f, "{}", c)
    }
}

fn bfs(map: &[Vec<Element>]) -> Vec<Pos> {
    // Find the start point
    let map_size = (map.len(), map[0].len());

    let start_pos = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, elem)| {
                if let Element::Start = elem {
                    Some(Pos {
                        x: i as i32,
                        y: j as i32,
                    })
                } else {
                    None
                }
            })
        })
        .expect("Start point not found");

    let mut frontier = VecDeque::<Pos>::new();
    let mut visited = HashSet::<Pos>::new();
    let mut parent_map = HashMap::<Pos, Option<Pos>>::new();

    parent_map.insert(start_pos, None);

    frontier.push_back(start_pos);

    while let Some(current) = frontier.pop_front() {
        visited.insert(current);

        if map[current.x as usize][current.y as usize] == Element::End {
            break;
        }

        for dir in DIRECTIONS {
            let neighbor = current + dir.to_pos();

            if !in_bound(&neighbor, map_size)
                || map[neighbor.x as usize][neighbor.y as usize] == Element::Wall
            {
                continue;
            }

            if visited.contains(&neighbor) {
                continue;
            }

            parent_map.insert(neighbor, Some(current));

            frontier.push_back(neighbor);
        }
    }

    // Backtrace
    let mut current = parent_map
        .iter()
        .find(|(pos, _)| map[pos.x as usize][pos.y as usize] == Element::End)
        .map(|(pos, _)| *pos)
        .expect("End point not found");

    let mut path = Vec::<Pos>::new();

    path.push(current);
    while let Some(Some(pos)) = parent_map.get(&current) {
        path.push(*pos);
        current = *pos;
    }
    path.reverse();

    path
}

fn part_1(map: &[Vec<Element>]) -> Result<()> {
    let path = bfs(map);

    let mut count = 0;

    for (idx, pos) in path.iter().enumerate() {
        for neighbor in path.iter().skip(idx + 100 + 2) {
            if (neighbor.x - pos.x).abs() + (neighbor.y - pos.y).abs() <= 2 {
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);

    Ok(())
}

fn part_2(map: &[Vec<Element>]) -> Result<()> {
    let path = bfs(map);

    let mut count_map = HashMap::<i32, usize>::new();

    for (idx, pos) in path.iter().enumerate() {
        for (neighbor_idx, neighbor) in path.iter().enumerate().skip(idx + 100) {
            let used_time = (neighbor.x - pos.x).abs() + (neighbor.y - pos.y).abs();
            if used_time <= 20 {
                count_map
                    .entry(neighbor_idx as i32 - idx as i32 - used_time)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    let count = count_map
        .iter()
        .filter(|x| *x.0 >= 100)
        .map(|x| x.1)
        .sum::<usize>();

    println!("Part 2: {}", count);

    Ok(())
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_20.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let map = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Element::Wall,
                    '.' => Element::Empty,
                    'S' => Element::Start,
                    'E' => Element::End,
                    _ => panic!("Unexpected character: {}", c),
                })
                .collect::<Vec<Element>>()
        })
        .collect::<Vec<Vec<Element>>>();

    part_1(&map)?;
    part_2(&map)?;

    Ok(())
}
