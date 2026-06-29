use anyhow::{Context, Result};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Debug)]
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn parse_a_line(line: &str, split_char: char) -> Result<(i64, i64)> {
    let arrs = line.split_whitespace().collect::<Vec<_>>();

    let len = arrs.len();

    let num_1 = arrs[len - 2]
        .split(split_char)
        .next_back()
        .context("Parse first number split 1")?
        .split(",")
        .next()
        .context("Parse first number split 2")?
        .parse::<i64>()
        .context("Parse first number into i64")?;
    let num_2 = arrs[len - 1]
        .split(split_char)
        .next_back()
        .context("Parse second number split 1")?
        .parse::<i64>()
        .context("Parse second number into i64")?;

    Ok((num_1, num_2))
}

#[allow(dead_code)]
fn solve_game(game: &Game) -> Option<(i64, i64)> {
    // println!(
    //     "Solving game with button A: {:?}, button B: {:?}, prize: {:?}",
    //     game.button_a, game.button_b, game.prize
    // );

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Item {
        point: i64,
        pushes: (i64, i64),
        nums: (i64, i64),
    }

    // Map points -> pushes
    let mut map: HashMap<(i64, i64), (i64, i64)> = HashMap::new();

    let mut frontier = BinaryHeap::new();

    frontier.push(Reverse(Item {
        point: 0,
        pushes: (0, 0),
        nums: (0, 0),
    }));

    while !frontier.is_empty() {
        let Item {
            point: _current_point,
            pushes: current_pushes,
            nums: current_nums,
        } = frontier.pop().unwrap().0;

        // println!(
        //     "Current nums: {:?}, current pushes: {:?}",
        //     current_nums, current_pushes
        // );

        if current_nums == game.prize {
            return Some(current_pushes);
        }

        if current_nums.0 > game.prize.0 || current_nums.1 > game.prize.1 {
            continue;
        }

        if map.contains_key(&current_nums) {
            continue;
        }
        map.insert(current_nums, current_pushes);

        let remaining_points = (game.prize.0 - current_nums.0, game.prize.1 - current_nums.1);

        if let Some(existing_pushes) = map.get(&remaining_points) {
            return Some((
                current_pushes.0 + existing_pushes.0,
                current_pushes.1 + existing_pushes.1,
            ));
        }

        let next_num_a = (
            current_nums.0 + game.button_a.0,
            current_nums.1 + game.button_a.1,
        );

        let next_pushes_a = (current_pushes.0 + 1, current_pushes.1);

        frontier.push(Reverse(Item {
            point: 3 * next_pushes_a.0 + next_pushes_a.1,
            pushes: next_pushes_a,
            nums: next_num_a,
        }));

        let next_num_b = (
            current_nums.0 + game.button_b.0,
            current_nums.1 + game.button_b.1,
        );

        let next_pushes_b = (current_pushes.0, current_pushes.1 + 1);

        frontier.push(Reverse(Item {
            point: 3 * next_pushes_b.0 + next_pushes_b.1,
            pushes: next_pushes_b,
            nums: next_num_b,
        }));
    }

    None
}

fn solve_game_analytically(game: &Game) -> Option<(i128, i128)> {
    let ax = game.button_a.0 as i128;
    let ay = game.button_a.1 as i128;
    let bx = game.button_b.0 as i128;
    let by = game.button_b.1 as i128;
    let px = game.prize.0 as i128;
    let py = game.prize.1 as i128;

    let det = ax * by - ay * bx;
    assert!(det != 0, "Determinant is zero, cannot solve");

    let a_num = px * by - py * bx;
    let b_num = ax * py - ay * px;

    if a_num % det != 0 || b_num % det != 0 {
        return None;
    }

    let a = a_num / det;
    let b = b_num / det;

    if a < 0 || b < 0 {
        return None;
    }

    Some((a, b))
}

fn part_1(games: &[Game]) -> Result<()> {
    let count = games
        .iter()
        .map(|game| solve_game_analytically(game).unwrap_or((0, 0)))
        // .inspect(|x| {
        //     println!("Game result: {:?}", x);
        // })
        .map(|(a, b)| 3 * a + b)
        .sum::<i128>();

    println!("Part 1: {}", count);

    Ok(())
}

fn part_2(games: &[Game]) -> Result<()> {
    let count = games
        .iter()
        .map(|game| solve_game_analytically(game).unwrap_or((0, 0)))
        // .inspect(|x| {
        //     println!("Game result: {:?}", x);
        // })
        .map(|(a, b)| 3 * a + b)
        .sum::<i128>();

    println!("Part 2: {}", count);

    Ok(())
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_13.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let mut games = lines
        .chunks(4)
        .map(|arrs| {
            let button_a = parse_a_line(arrs[0], '+')?;
            let button_b = parse_a_line(arrs[1], '+')?;
            let prize = parse_a_line(arrs[2], '=')?;

            // println!(
            //     "Button A: {:?}, Button B: {:?}, Prize: {:?}",
            //     button_a, button_b, prize
            // );

            Ok(Game {
                button_a,
                button_b,
                prize,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    part_1(&games)?;

    games.iter_mut().for_each(|game| {
        game.prize.0 += 10000000000000;
        game.prize.1 += 10000000000000;
    });

    part_2(&games)?;

    Ok(())
}
