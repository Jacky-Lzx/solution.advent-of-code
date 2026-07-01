use anyhow::{Context, Result};
use std::{collections::HashSet, fs};

use advent_of_code::utils::pos::{DIRECTIONS, Pos};

enum Measure {
    Single(HashSet<Pos>),
    Distinct(u32),
}

fn score(mountain: &[Vec<u32>], start: Pos, is_distinct: bool) -> Result<u32> {
    let mut frontier = vec![start];

    let mut measure = if is_distinct {
        Measure::Distinct(0)
    } else {
        Measure::Single(HashSet::new())
    };

    while !frontier.is_empty() {
        let head = frontier.pop().context("pop from frontier")?;

        if mountain[head.x as usize][head.y as usize] == 9 {
            match measure {
                Measure::Single(ref mut set) => {
                    set.insert(head);
                }
                Measure::Distinct(ref mut count) => {
                    *count += 1;
                }
            }
            continue;
        }

        for dir in DIRECTIONS {
            let next = head + dir.to_pos();

            if next.x < 0
                || next.y < 0
                || next.x as usize >= mountain.len()
                || next.y as usize >= mountain[0].len()
            {
                continue;
            }

            let next_height = mountain[next.x as usize][next.y as usize];
            let head_height = mountain[head.x as usize][head.y as usize];

            if next_height != head_height + 1 {
                continue;
            }

            frontier.push(next);
        }
    }

    match measure {
        Measure::Single(set) => Ok(set.len() as u32),
        Measure::Distinct(count) => Ok(count),
    }
}

fn part_1(mountain: &[Vec<u32>]) -> Result<()> {
    let mut count = 0;

    mountain.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, &height)| {
            if height != 0 {
                return;
            }

            let pos = Pos {
                x: x as i32,
                y: y as i32,
            };

            if let Ok(s) = score(mountain, pos, false) {
                // println!("Score for position ({}, {}): {}", x, y, s);
                count += s;
            }
        });
    });

    println!("Part 1: {}", count);

    Ok(())
}

fn part_2(mountain: &[Vec<u32>]) -> Result<()> {
    let mut count = 0;

    mountain.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, &height)| {
            if height != 0 {
                return;
            }

            let pos = Pos {
                x: x as i32,
                y: y as i32,
            };

            if let Ok(s) = score(mountain, pos, true) {
                // println!("Score for position ({}, {}): {}", x, y, s);
                count += s;
            }
        });
    });

    println!("Part 2: {}", count);

    Ok(())
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_10.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let mountain = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .with_context(|| format!("{} is not a valid digit", c))
                })
                .collect::<Result<Vec<u32>>>()
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;

    part_1(&mountain)?;
    part_2(&mountain)?;

    Ok(())
}
