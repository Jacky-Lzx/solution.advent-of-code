use advent_of_code::utils::pos::{DIRECTIONS, Pos};
use anyhow::{Context, Result};
use std::{collections::VecDeque, fs};

enum CountType {
    Perimeter,
    Side,
}

fn get_upper_or_left(garden: &[Vec<char>], current: Pos, dir_idx: usize, size: (i32, i32)) -> u32 {
    // Map direction index to the direction that need to be checked for upper or left
    let idx_map = [(2, 0), (0, 1), (0, 2), (2, 3)];

    let current_letter = garden[current.x as usize][current.y as usize];

    let idxes = idx_map[dir_idx];

    let new_1 = current + DIRECTIONS[idxes.0];

    if new_1.x < 0
        || new_1.y < 0
        || new_1.x >= size.0
        || new_1.y >= size.1
        || current_letter != garden[new_1.x as usize][new_1.y as usize]
    {
        return 1;
    }

    let new_2 = new_1 + DIRECTIONS[idxes.1];

    if new_2.x < 0
        || new_2.y < 0
        || new_2.x >= size.0
        || new_2.y >= size.1
        || current_letter != garden[new_2.x as usize][new_2.y as usize]
    {
        0
    } else {
        1
    }
}

fn bfs(
    garden: &[Vec<char>],
    start: Pos,
    visited: &mut [Vec<bool>],
    count_type: CountType,
) -> Result<(u32, u32)> {
    let (x_len, y_len) = (garden.len() as i32, garden[0].len() as i32);

    let mut frontier = VecDeque::from(vec![start]);

    let letter = garden[start.x as usize][start.y as usize];

    let mut area = 0;
    let mut count = 0;

    while !frontier.is_empty() {
        let current = frontier.pop_front().context("Pop from frontier")?;

        if visited[current.x as usize][current.y as usize] {
            continue;
        }

        area += 1;

        visited[current.x as usize][current.y as usize] = true;

        for (dir_idx, dir) in DIRECTIONS.iter().enumerate() {
            let next = current + *dir;

            if next.x < 0 || next.x >= x_len || next.y < 0 || next.y >= y_len {
                count += match count_type {
                    CountType::Perimeter => 1,
                    CountType::Side => get_upper_or_left(garden, current, dir_idx, (x_len, y_len)),
                };
                continue;
            }

            if garden[next.x as usize][next.y as usize] == letter {
                frontier.push_back(next);
            } else {
                count += match count_type {
                    CountType::Perimeter => 1,
                    CountType::Side => get_upper_or_left(garden, current, dir_idx, (x_len, y_len)),
                };
            }
        }
    }

    Ok((area, count))
}

fn part_1(garden: &[Vec<char>]) -> Result<()> {
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];

    let mut count = 0;

    for (x, line) in garden.iter().enumerate() {
        for (y, _point) in line.iter().enumerate() {
            if !visited[x][y] {
                let (area, perimeter) = bfs(
                    garden,
                    Pos {
                        x: x as i32,
                        y: y as i32,
                    },
                    &mut visited,
                    CountType::Perimeter,
                )?;
                // println!("Point: {}, Area: {}, Perimeter: {}", point, area, perimeter);

                count += area * perimeter;
            }
        }
    }

    println!("Part 1: {}", count);

    Ok(())
}

fn part_2(garden: &[Vec<char>]) -> Result<()> {
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];

    let mut count = 0;

    for (x, line) in garden.iter().enumerate() {
        for (y, _point) in line.iter().enumerate() {
            if !visited[x][y] {
                let (area, perimeter) = bfs(
                    garden,
                    Pos {
                        x: x as i32,
                        y: y as i32,
                    },
                    &mut visited,
                    CountType::Side,
                )?;

                // println!("Point: {}, Area: {}, Perimeter: {}", point, area, perimeter);
                // visited.iter().for_each(|line| {
                //     line.iter().for_each(|&v| {
                //         print!("{}", if v { "1" } else { "0" });
                //     });
                //     println!();
                // });
                // println!();

                count += area * perimeter;
            }
        }
    }

    println!("Part 2: {}", count);

    Ok(())
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_12.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let garden = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    part_1(&garden)?;
    part_2(&garden)?;

    Ok(())
}
