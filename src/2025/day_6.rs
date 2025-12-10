use std::fs::read_to_string;

fn part_1(contents: &[&str]) {
    let len = contents.len() - 1;

    let operands = contents[0..len - 1]
        .iter()
        .map(|&x| {
            // println!("x: {}", x);
            x.trim()
                .split_ascii_whitespace()
                .map(|x| x.parse::<i128>().unwrap())
                .collect::<Vec<i128>>()
        })
        .collect::<Vec<Vec<i128>>>();

    let operators = contents[len - 1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();

    let mut result = 0;

    for i in 0..operators.len() {
        match operators[i] {
            "+" => {
                let mut result_t = 0;
                for operand in &operands {
                    result_t += operand[i];
                }
                result += result_t;
            }
            "*" => {
                let mut result_t = 1;

                for operand in &operands {
                    result_t *= operand[i];
                }
                result += result_t;
            }
            _ => panic!("Unknown operator: {}", operators[i]),
        }
    }

    println!("Part 1: {}", result);
}

fn part_2(contents: &[&str]) {
    let len = contents.len() - 1;

    let mut operands_new = vec![vec![]];
    let len_operands = contents[0..len - 1].iter().map(|&x| x.len()).max().unwrap();
    let mut operand_index = 0;

    for i in 0..len_operands {
        let mut operand = 0;
        let mut zero_skip = 0;

        for content in &contents[0..len - 1] {
            let row = content.chars().nth(i).unwrap_or('0');

            if row == ' ' {
                zero_skip += 1;
                continue;
            } else {
                if operand == 0 {
                    operand = row.to_digit(10).unwrap() as i128;
                } else {
                    operand = operand * 10_i128.pow(zero_skip as u32 + 1)
                        + row.to_digit(10).unwrap() as i128;
                }
                zero_skip = 0;
            }
        }
        if operand == 0 {
            operand_index += 1;
            // println!("Current operands: {:?}", operands_new[operand_index - 1]);
            continue;
        }

        if operands_new.len() <= operand_index {
            operands_new.push(vec![]);
        }

        operands_new[operand_index].push(operand);
    }

    let operators = contents[len - 1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();

    let mut result = 0;

    for i in 0..operators.len() {
        match operators[i] {
            "+" => {
                result += operands_new[i].iter().sum::<i128>();
            }
            "*" => {
                result += operands_new[i].iter().product::<i128>();
            }
            _ => panic!("Unknown operator: {}", operators[i]),
        }
    }

    println!("Part 2: {}", result);
}

fn main() {
    let contents = read_to_string("assets/2025/day_6.input").unwrap();
    // let contents = read_to_string("assets/2025/test.input").unwrap();

    let contents = contents.split('\n').collect::<Vec<&str>>();

    part_1(&contents);
    part_2(&contents);
}
