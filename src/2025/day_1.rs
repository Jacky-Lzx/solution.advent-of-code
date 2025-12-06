use std::fs;

enum Action {
    Left(u32),
    Right(u32),
}

fn part_1(actions: &[Action]) {
    let mut count = 0;
    let mut cur: i32 = 50;
    for action in actions {
        match action {
            Action::Left(n) => cur -= *n as i32,
            Action::Right(n) => cur += *n as i32,
        }
        if cur % 100 == 0 {
            count += 1;
        }
    }

    println!("Part 1: {}", count);
}

fn part_2(actions: &[Action]) {
    let mut count = 0;
    let mut cur: i32 = 50;
    for action in actions {
        let cur_p = cur;
        match action {
            Action::Left(n) => cur -= *n as i32,
            Action::Right(n) => cur += *n as i32,
        }

        // If previous position was 0 and current is negative, we do not cross 0
        if cur_p == 0 && cur < 0 {
            count -= 1;
        }

        while cur < 0 {
            cur += 100;
            count += 1;
        }

        if cur == 0 {
            count += 1;
        }

        while cur >= 100 {
            cur -= 100;
            count += 1;
        }
    }

    println!("Part 2: {}", count);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("assets/2025/day_1.input")?;
    // println!("File contents:\n{}", contents);

    let actions: Vec<&str> = contents.lines().collect();
    let actions = actions
        .iter()
        .map(|s| {
            let t = s.split_at(1);
            match t.0 {
                "L" => t.1.parse::<u32>().map(Action::Left).map_err(|e| e.into()),
                "R" => t.1.parse::<u32>().map(Action::Right).map_err(|e| e.into()),
                _ => Err("Unknown action".into()),
            }
        })
        .collect::<Result<Vec<Action>, Box<dyn std::error::Error>>>()?;

    part_1(&actions);
    part_2(&actions);

    Ok(())
}
