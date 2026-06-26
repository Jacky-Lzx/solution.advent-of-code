use std::fs;

fn dfs_1(target: u64, values: &[u64], idx: usize, accum: u64) -> bool {
    if idx == values.len() {
        return accum == target;
    }

    let mult = accum * values[idx];
    let add = accum + values[idx];

    (mult <= target && dfs_1(target, values, idx + 1, mult))
        || (add <= target && dfs_1(target, values, idx + 1, add))
}

fn dfs_2(target: u64, values: &[u64], idx: usize, accum: u64) -> bool {
    if idx == values.len() {
        return accum == target;
    }

    let mult = accum * values[idx];
    let add = accum + values[idx];
    let concat = accum * 10u64.pow(values[idx].ilog10() + 1) + values[idx];

    (mult <= target && dfs_2(target, values, idx + 1, mult))
        || (add <= target && dfs_2(target, values, idx + 1, add))
        || (concat <= target && dfs_2(target, values, idx + 1, concat))
}

fn part_1(equations: &[(u64, Vec<u64>)]) {
    let sum: u64 = equations
        .iter()
        .filter(|(target, values)| dfs_1(*target, values, 0, 0))
        .map(|(target, _)| target)
        .sum();

    println!("Part 1: {}", sum);
}

fn part_2(equations: &[(u64, Vec<u64>)]) {
    let sum: u64 = equations
        .iter()
        .filter(|(target, values)| dfs_2(*target, values, 0, 0))
        .map(|(target, _)| target)
        .sum();

    println!("Part 2: {}", sum);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("assets/2024/day_7.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let equations: Vec<(u64, Vec<u64>)> = lines
        .iter()
        .map(|line| {
            let mut iter = line.split(":");
            (
                iter.next().unwrap().parse::<u64>().unwrap(),
                iter.next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .collect();

    part_1(&equations);
    part_2(&equations);

    Ok(())
}
