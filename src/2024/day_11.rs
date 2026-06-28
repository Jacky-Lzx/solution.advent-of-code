use anyhow::{Context, Result};
use std::{collections::HashMap, fs};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Outcome {
    One(u64),
    Two(u64, u64),
}

fn transform(stone: u64) -> Outcome {
    if stone == 0 {
        return Outcome::One(1);
    }

    let digits = stone.ilog10() + 1;

    if digits.is_multiple_of(2) {
        let divisor = 10u64.pow(digits / 2);
        return Outcome::Two(stone / divisor, stone % divisor);
    }

    Outcome::One(stone * 2024)
}

fn blink(stones: &[u64], time: u32, count_map: &mut HashMap<(u64, u32), u64>) -> u64 {
    if time == 0 {
        return 1;
    }

    let mut count = 0;
    for &stone in stones {
        if let Some(&c) = count_map.get(&(stone, time)) {
            count += c;
            continue;
        }

        match transform(stone) {
            Outcome::One(o) => {
                let c = blink(&[o], time - 1, count_map);
                count += c;
                count_map.insert((o, time - 1), c);
            }
            Outcome::Two(a, b) => {
                let c = blink(&[a], time - 1, count_map);
                count += c;
                count_map.insert((a, time - 1), c);

                let c = blink(&[b], time - 1, count_map);
                count += c;
                count_map.insert((b, time - 1), c);
            }
        };
    }

    count
}

fn part_1(stones: &[u64]) -> Result<()> {
    let mut map = HashMap::new();
    println!("Part 1: {}", blink(stones, 25, &mut map));

    Ok(())
}

fn part_2(stones: &[u64]) -> Result<()> {
    let mut map = HashMap::new();
    println!("Part 2: {}", blink(stones, 75, &mut map));

    Ok(())
}

#[test]
fn test_transform() {
    assert_eq!(transform(99), Outcome::Two(9, 9));
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_11.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let stones = lines[0]
        .split(' ')
        .map(|x| x.parse::<u64>().context("Parse string to number"))
        .collect::<Result<Vec<u64>>>()?;

    part_1(&stones)?;
    part_2(&stones)?;

    Ok(())
}
