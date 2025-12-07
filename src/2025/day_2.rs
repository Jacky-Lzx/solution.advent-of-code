use std::fs;

struct Range {
    start: u128,
    end: u128,
}

fn part_1(ranges: &Vec<Range>) {
    let mut result_num: u128 = 0;

    for range in ranges {
        println!("Parsed range: {} - {}", range.start, range.end);

        for num in range.start..=range.end {
            if num == 0 {
                continue;
            }

            // print!("{} ", num);
            let digit_num = num.ilog10() + 1;

            if digit_num % 2 != 0 {
                continue;
            }

            // println!("Digits: {}", digit_num);

            let divisor: u128 = 10_i32.pow(digit_num / 2) as u128;
            if num % divisor == (num / divisor) {
                result_num = result_num.saturating_add(num);

                println!("{}", num);
            }
        }
    }

    println!("Part 1: {}", result_num);
}

fn part_2(ranges: &Vec<Range>) {
    let mut result_num: u128 = 0;

    for range in ranges {
        println!("Parsed range: {} - {}", range.start, range.end);

        for num in range.start..=range.end {
            if num == 0 {
                continue;
            }

            // print!("{} ", num);
            let digit_num = num.ilog10() + 1;

            if digit_num == 1 {
                continue;
            }

            let mut repeated = false;

            for i in 1..=(digit_num / 2) {
                let divisor = 10_i32.pow(i) as u128;
                let repeated_num = num % divisor;
                // Check the last org_num and the repeated_num
                // To filter out the number like "70707"
                if repeated_num < divisor / 10 {
                    continue;
                }

                let mut org_num = num;

                let mut repeated_local = true;
                while org_num > 0 {
                    if org_num % divisor != repeated_num {
                        repeated_local = false;
                        break;
                    }
                    org_num /= divisor;
                }

                if repeated_local {
                    repeated = true;
                    break;
                }
            }

            if repeated {
                println!("{}", num);
                result_num += num;
            }
        }
    }

    println!("Part 2: {}", result_num);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("assets/2025/day_2.input")?;
    // let contents = fs::read_to_string("assets/2025/test.input")?;
    println!("File contents:\n{}", contents);

    let ranges: Vec<Range> = contents
        .split(',')
        .map(|part| {
            let bounds: Vec<&str> = part.trim().split('-').collect();
            if bounds.len() == 2 {
                let start: u128 = bounds[0].parse().unwrap();
                let end: u128 = bounds[1].parse().unwrap();
                let range = Range { start, end };
                Ok(range)
            } else {
                Err("Invalid range format")
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    part_1(&ranges);

    part_2(&ranges);

    Ok(())
}
