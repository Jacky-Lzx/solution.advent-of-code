use std::fs;

fn part_1(batteries: &Vec<&str>) {
    let mut result: i64 = 0;
    for battery in batteries {
        println!("Battery: {}", battery);

        let len = battery.len();

        let mut maximum = -1;
        let mut second = -1;
        for (i, ch) in battery.chars().enumerate() {
            let digit = ch.to_digit(10).unwrap() as i64;
            if digit > maximum && i != len - 1 {
                maximum = digit;
                second = -1;
            } else if digit > second {
                second = digit;
            }
        }

        result += maximum * 10 + second;

        println!("Number: {}", maximum * 10 + second);
    }
    println!("Part 1: {}", result)
}

fn part_2(batteries: &Vec<&str>) {
    const SIZE: usize = 12;

    let mut result: i64 = 0;
    for battery in batteries {
        println!("Battery: {}", battery);

        let len = battery.len();

        let mut digits = [-1; SIZE];

        let mut index = 0;

        for (ch_i, ch) in battery.chars().enumerate() {
            let d = ch.to_digit(10).unwrap() as i64;
            for j in (0..=index).rev() {
                if d > digits[j] && ch_i <= len - SIZE + j {
                    digits[j] = d;

                    if j < SIZE - 1 {
                        digits[j + 1] = -1;
                    }

                    index = j;
                }
            }

            index = (index + 1).min(SIZE - 1);
        }

        let mut total = 0;
        for d in digits.iter() {
            assert!(*d != -1);

            total *= 10;
            total += d;
        }

        result += total;
        println!("Number: {}", total);
    }
    println!("Part 2: {}", result);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("assets/2025/day_3.input")?;
    // let contents = fs::read_to_string("assets/2025/test.input")?;

    let batteries: Vec<&str> = contents.lines().collect();

    part_1(&batteries);
    part_2(&batteries);

    Ok(())
}
