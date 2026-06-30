use advent_of_code::utils::pos::Pos;
use anyhow::Result;
use std::{
    fs,
    io::{self, Write},
};

#[derive(Clone, Debug)]
struct Velocity(i32, i32);

fn part_1(robots: &[(Pos, Velocity)]) -> Result<()> {
    let map_size = (101, 103);
    // let map_size = (11, 7);
    let mut robots = robots.to_vec();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot_move(robot, map_size);
        }
    }

    let (a, b, c, d) = robots.iter().fold((0, 0, 0, 0), |nums, robot| {
        let mut change = (0, 0, 0, 0);
        if robot.0.x < map_size.0 / 2 {
            if robot.0.y < map_size.1 / 2 {
                change = (1, 0, 0, 0)
            } else if robot.0.y > map_size.1 / 2 {
                change = (0, 0, 1, 0);
            }
        } else if robot.0.x > map_size.0 / 2 {
            if robot.0.y < map_size.1 / 2 {
                change = (0, 1, 0, 0);
            } else if robot.0.y > map_size.1 / 2 {
                change = (0, 0, 0, 1);
            }
        }

        (
            nums.0 + change.0,
            nums.1 + change.1,
            nums.2 + change.2,
            nums.3 + change.3,
        )
    });

    let result = a * b * c * d;

    println!("Part 1: {}", result);

    Ok(())
}

fn robot_move(robot: &mut (Pos, Velocity), map_size: (i32, i32)) {
    let new_x = (robot.0.x + robot.1.0) % map_size.0;
    let new_y = (robot.0.y + robot.1.1) % map_size.1;

    if new_x < 0 {
        robot.0.x = map_size.0 + new_x;
    } else {
        robot.0.x = new_x;
    }
    if new_y < 0 {
        robot.0.y = map_size.1 + new_y;
    } else {
        robot.0.y = new_y;
    }
}

fn print_tree_and_pause(map: &[Vec<i32>]) {
    map.iter().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|&x| if x > 0 { '#' } else { '.' })
                .collect::<String>()
        );
    });
    println!();

    print!("Press Enter to continue...");
    // Flush stdout to guarantee the prompt prints immediately
    io::stdout().flush().unwrap();

    // Read input until a newline character is encountered
    let mut _dummy = String::new();
    io::stdin().read_line(&mut _dummy).unwrap();
}

// The idea is to find a 3x3 square of rebots that are all occupied. Then print the map and check it
// manually.
// Refer to `https://www.reddit.com/r/adventofcode/comments/1hdw2m1/2024_day_14_part_2`
fn part_2(robots: &[(Pos, Velocity)]) -> Result<()> {
    let map_size = (101, 103);
    // let map_size = (11, 7);
    let mut robots = robots.to_vec();

    'outer: for iter_idx in 0..(map_size.0 * map_size.1) {
        let mut map = vec![vec![0; map_size.0 as usize]; map_size.1 as usize];

        for robot in robots.iter_mut() {
            robot_move(robot, map_size);

            map[robot.0.y as usize][robot.0.x as usize] += 1;
        }

        const DIRECTIONS: [(i32, i32); 8] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];

        for y in 1..(map_size.1 - 1) {
            for x in 1..(map_size.0 - 1) {
                if map[y as usize][x as usize] > 0 {
                    let mut flag = true;
                    for (dx, dy) in DIRECTIONS.iter() {
                        let neighbor_x = x + dx;
                        let neighbor_y = y + dy;
                        if map[neighbor_y as usize][neighbor_x as usize] == 0 {
                            flag = false;
                            break;
                        }
                    }
                    if flag {
                        println!(
                            "Iteration {}: Found a robot at ({}, {}) with all neighbors occupied.",
                            iter_idx + 1,
                            x,
                            y
                        );
                        print_tree_and_pause(&map);
                        continue 'outer;
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_14.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let robots = lines
        .iter()
        .map(|line| {
            let arrs = line.split(' ').collect::<Vec<&str>>();

            let position = arrs[0].split('=').collect::<Vec<&str>>();
            let position = position[1].split(',').collect::<Vec<&str>>();
            let position = Pos {
                x: position[0].parse::<i32>().expect("Invalid number"),
                y: position[1].parse::<i32>().expect("Invalid number"),
            };

            let velocity = arrs[1].split('=').collect::<Vec<&str>>();
            let velocity = velocity[1].split(',').collect::<Vec<&str>>();
            let velocity = Velocity(
                velocity[0].parse::<i32>().expect("Invalid number"),
                velocity[1].parse::<i32>().expect("Invalid number"),
            );

            (position, velocity)
        })
        .collect::<Vec<(Pos, Velocity)>>();

    part_1(&robots)?;
    part_2(&robots)?;

    Ok(())
}
