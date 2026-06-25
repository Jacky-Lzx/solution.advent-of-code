use std::fs;

fn part_1(left: &[i32], right: &[i32]) {
    let mut sum = 0;

    left.iter().enumerate().for_each(|(i, &l)| {
        sum += (l - right[i]).abs();
    });

    println!("Part 1: {}", sum);
}

fn part_2(left: &[i32], right: &[i32]) {
    let mut sum = 0;

    let mut right_index = 0;

    for l in left {
        while right_index < right.len() && right[right_index] < *l {
            right_index += 1;
        }

        let mut count = 0;

        while right_index < right.len() && right[right_index] == *l {
            count += 1;
            right_index += 1;
        }

        sum += *l * count;
    }

    println!("Part 2: {}", sum);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("assets/2024/day_1.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let (mut left, mut right): (Vec<_>, Vec<_>) = lines
        .iter()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            (nums[0], nums[1])
        })
        .unzip();

    left.sort();
    right.sort();

    part_1(&left, &right);

    part_2(&left, &right);

    Ok(())
}
