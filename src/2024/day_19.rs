use anyhow::Result;
use std::{collections::HashMap, fs};

struct Towels {
    end: bool,
    next: [Option<Box<Towels>>; 26],
}

impl Towels {
    pub fn new() -> Self {
        Self {
            end: false,
            next: Default::default(),
        }
    }

    pub fn add(&mut self, towel: &str) {
        if towel.is_empty() {
            self.end = true;
            return;
        }

        let c = towel.chars().next().unwrap();

        let idx = c as usize - 'a' as usize;

        if self.next[idx].is_none() {
            self.next[idx] = Some(Box::new(Towels::new()));
        }

        self.next[idx].as_mut().unwrap().add(&towel[1..]);
    }

    pub fn find(&self, target: &str) -> bool {
        if target.is_empty() {
            return self.end;
        }

        let c = target.chars().next().unwrap();
        let idx = c as usize - 'a' as usize;

        if let Some(next_towel) = &self.next[idx] {
            return next_towel.find(&target[1..]);
        }

        false
    }

    pub fn is_prefix(&self, target: &str) -> bool {
        if target.is_empty() {
            return true;
        }

        let c = target.chars().next().unwrap();
        let idx = c as usize - 'a' as usize;

        if let Some(next_towel) = &self.next[idx] {
            return next_towel.is_prefix(&target[1..]);
        }

        false
    }
}

fn find_towels(towels: &Towels, target: &str, idx: usize) -> bool {
    // println!("find_towels: idx = {}, target = {}", idx, target);

    if idx == target.len() {
        return true;
    }

    for i in idx..target.len() {
        if !towels.is_prefix(&target[idx..=i]) {
            break;
        }

        if towels.find(&target[idx..=i]) && find_towels(towels, target, i + 1) {
            return true;
        }
    }

    false
}

fn get_towel_nums(towels: &Towels, target: &str, idx: usize, dp: &mut HashMap<String, u64>) -> u64 {
    if idx == target.len() {
        return 1;
    }

    if let Some(&n) = dp.get(&target[idx..]) {
        return n;
    }

    let mut sum = 0;

    for i in idx..target.len() {
        if !towels.is_prefix(&target[idx..=i]) {
            break;
        }

        if towels.find(&target[idx..=i]) {
            sum += get_towel_nums(towels, target, i + 1, dp);
        }
    }

    dp.insert(target[idx..].to_string(), sum);

    sum
}

fn part_2(towels: &Towels, targets: &[&str]) -> Result<()> {
    let num = targets
        .iter()
        .map(|t| get_towel_nums(towels, t, 0, &mut HashMap::new()))
        .sum::<u64>();

    println!("Part 2: {}", num);

    Ok(())
}

fn part_1(towels: &Towels, targets: &[&str]) -> Result<()> {
    let num = targets.iter().filter(|t| find_towels(towels, t, 0)).count();

    println!("Part 1: {}", num);

    Ok(())
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_19.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let mut line_it = contents.lines();

    let towels_raw = line_it.next().unwrap().split(", ").collect::<Vec<&str>>();

    line_it.next();

    let targets = line_it.collect::<Vec<&str>>();

    let mut towels = Towels::new();
    for towel in towels_raw {
        towels.add(towel);
    }

    part_1(&towels, &targets)?;
    part_2(&towels, &targets)?;

    Ok(())
}
