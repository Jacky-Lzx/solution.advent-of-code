use std::fs;

use regex::Regex;

fn part_1(lines: &[&str]) {
    let mul_regex = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();

    let total = lines
        .iter()
        .map(|line| {
            mul_regex
                .captures_iter(line)
                .map(|cap| {
                    let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    a * b
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Part 1: {}", total);
}

fn part_2(lines: &[&str]) {
    let mul_regex = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();
    let enable = Regex::new(r"do\(\)").unwrap();
    let disable = Regex::new(r"don't\(\)").unwrap();

    let mut total = 0;

    // NOTE: The enable/disable status is preserved during line break <2026.06.26, lzx>
    let line = &lines.join("")[..];

    let mut total_line = 0;
    let mut line_remaining = line;

    loop {
        if let Some(disable_match) = disable.find(line_remaining) {
            let line_before_disable = &line_remaining[..disable_match.start()];
            mul_regex
                .captures_iter(line_before_disable)
                .for_each(|cap| {
                    let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    println!("a: {}, b: {}", a, b);
                    total_line += a * b;
                });
            line_remaining = &line_remaining[disable_match.end()..];
            if let Some(enable_match) = enable.find(line_remaining) {
                line_remaining = &line_remaining[enable_match.end()..];
            } else {
                // Do not find do() in the remaining line
                break;
            }
        } else {
            // Do not find don't() in the remaining line
            mul_regex.captures_iter(line_remaining).for_each(|cap| {
                let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                println!("a: {}, b: {}", a, b);
                total_line += a * b;
            });
            break;
        }
    }

    total += total_line;

    println!("Part 2: {}", total);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("assets/2024/day_3.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines = contents.lines().collect::<Vec<&str>>();

    part_1(&lines);
    part_2(&lines);

    Ok(())
}
