use std::fs::read_to_string;

#[derive(Debug)]
struct Range {
    start: i128,
    end: i128,
}

fn part_1(ranges: &[Range], ids: &[i128]) {
    let mut count = 0;

    for id in ids {
        for range in ranges {
            if *id >= range.start && *id <= range.end {
                count += 1;
                break;
            }
        }
    }

    println!("Part 1: {}", count);
}

fn part_2(ranges: &mut [Range]) {
    ranges.sort_by_key(|x| x.start);

    let mut end_max = -1;

    let mut count = 0;

    for range in ranges {
        if range.start <= end_max {
            if range.end > end_max {
                count += range.end - end_max;
                end_max = range.end;
            }
        } else {
            count += range.end - range.start + 1;
            if range.end > end_max {
                end_max = range.end;
            }
        }
    }

    println!("Part 2: {}", count);
}

fn main() {
    let contents = read_to_string("assets/2025/day_5.input").unwrap();
    // let contents = read_to_string("assets/2025/test.input").unwrap();

    let contents = contents.lines().collect::<Vec<&str>>();

    let mut ranges = vec![];
    let mut ids = vec![];

    let mut parse_range = true;

    for content in contents {
        if content.is_empty() {
            parse_range = false;
            continue;
        }

        if parse_range {
            let mut it = content.split('-');
            ranges.push(Range {
                start: it.next().unwrap().parse::<i128>().unwrap(),
                end: it.next().unwrap().parse::<i128>().unwrap(),
            });
        } else {
            ids.push(content.parse::<i128>().unwrap());
        }
    }

    part_1(&ranges, &ids);
    part_2(&mut ranges);
}
