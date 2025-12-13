use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const START: char = 'S';
const SPLITTER: char = '^';

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Position(i128, i128);

#[derive(Debug)]
struct Splitter(Position);

fn part_1(start_pos: &Position, row_spliters_map: &HashMap<i128, Vec<Splitter>>) {
    let mut result = 0;

    let mut buffer: Vec<Position> = vec![start_pos.clone()];

    // Store the hit splitters for each row
    let mut hit_splitters: HashMap<i128, HashSet<i128>> = HashMap::new();

    while let Some(current_pos) = buffer.pop() {
        let pos_x = current_pos.0;
        let pos_y = current_pos.1;
        if let Some(splitters) = row_spliters_map.get(&pos_y)
            && let Some(splitter) = splitters.iter().find(|x| x.0.0 > pos_x)
        {
            if hit_splitters
                .entry(pos_y)
                .or_default()
                .iter()
                .any(|x| *x == splitter.0.0)
            {
                continue;
            }

            hit_splitters.entry(pos_y).or_default().insert(splitter.0.0);

            // println!("  Next Splitter at: {:?}", splitter.0);

            result += 1;

            let left = Position(splitter.0.0, splitter.0.1 - 1);

            buffer.push(left);

            let right = Position(splitter.0.0, splitter.0.1 + 1);
            buffer.push(right);
        };

        if !hit_splitters.contains_key(&pos_y)
            || hit_splitters.get(&pos_y).unwrap().iter().any(|x| *x == -1)
        {
            continue;
        }

        hit_splitters.entry(pos_y).or_default().insert(-1);
    }

    println!("Part 1: {}", result);
}

fn recursion(
    pos: &Position,
    map: &mut HashMap<Position, i128>,
    row_spliters_map: &HashMap<i128, Vec<Splitter>>,
) -> i128 {
    let mut result = 0;

    let pos_x = pos.0;
    let pos_y = pos.1;

    if let Some(splitters) = row_spliters_map.get(&pos_y)
        && let Some(splitter) = splitters.iter().find(|x| x.0.0 > pos_x)
    {
        // println!("  Next Splitter at: {:?}", splitter.0);
        let splitter_pos = Position(splitter.0.0, splitter.0.1);
        if map.contains_key(&splitter_pos) {
            return *map.get(&splitter_pos).unwrap();
        }

        result += 1;

        let left = Position(splitter.0.0 + 1, splitter.0.1 - 1);
        result += recursion(&left, map, row_spliters_map);

        let right = Position(splitter.0.0 + 1, splitter.0.1 + 1);
        result += recursion(&right, map, row_spliters_map);

        map.entry(splitter_pos).or_insert(result);
    };

    result
}

fn part_2(start_pos: &Position, row_spliters_map: &HashMap<i128, Vec<Splitter>>) {
    let mut result = 0;

    result += recursion(start_pos, &mut HashMap::new(), row_spliters_map);

    result += 1; // Starting position

    println!("Part 2: {}", result);
}

fn main() {
    let contents = read_to_string("assets/2025/day_7.input").unwrap();
    // let contents = read_to_string("assets/2025/test.input").unwrap();

    let contents = contents.lines().collect::<Vec<&str>>();

    let mut col_spliters_map = HashMap::<i128, Vec<Splitter>>::new();

    let mut start_pos = Position(-1, -1);

    for (x, content) in contents.iter().enumerate() {
        for (y, c) in content.chars().enumerate() {
            if c == SPLITTER {
                col_spliters_map
                    .entry(y as i128)
                    .or_default()
                    .push(Splitter(Position(x as i128, y as i128)));
            } else if c == START {
                start_pos = Position(x as i128, y as i128);
            }
        }
    }

    assert!(start_pos != Position(-1, -1), "Start position not found");
    // println!("{:?}", row_spliters_map);

    part_1(&start_pos, &col_spliters_map);
    part_2(&start_pos, &col_spliters_map);
}
