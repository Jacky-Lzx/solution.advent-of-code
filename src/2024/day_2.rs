use std::fs;

fn part_1(reports: &[Vec<i32>]) {
    let results: Vec<bool> = reports
        .iter()
        .map(|report| {
            report
                .windows(2)
                .all(|pair| (pair[1] - pair[0]).abs() >= 1 && (pair[1] - pair[0]).abs() <= 3)
                && (report.is_sorted() || report.is_sorted_by(|a, b| a >= b))
        })
        .collect();

    let num = results.iter().filter(|&x| *x).count();

    println!("Part 1: {}", num);
}

fn is_valid_once(report: &[i32]) -> bool {
    report
        .windows(2)
        .all(|pair| (pair[1] - pair[0]).abs() >= 1 && (pair[1] - pair[0]).abs() <= 3)
        && (report.is_sorted() || report.is_sorted_by(|a, b| a >= b))
}

fn part_2(reports: &[Vec<i32>]) {
    let mut num = 0;

    for report in reports {
        for i in 0..report.len() {
            let mut modified_report = report.clone();
            modified_report.remove(i);

            if is_valid_once(&modified_report) {
                num += 1;
                break;
            }
        }
    }

    println!("Part 2: {}", num);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("assets/2024/day_2.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines = contents.lines().collect::<Vec<&str>>();

    let reports: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    part_1(&reports);
    part_2(&reports);

    Ok(())
}
