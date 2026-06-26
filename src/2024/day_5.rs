use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part_1(rules_map: &HashMap<i32, HashSet<i32>>, updates: &[Vec<i32>]) {
    let is_valid: Vec<bool> = updates
        .iter()
        .map(|update| {
            update.is_sorted_by(|a, b| {
                rules_map.get(a).is_none_or(|rules| rules.contains(b))
                    && rules_map.get(b).is_none_or(|rules| !rules.contains(a))
            })
        })
        .collect();

    let count: i32 = updates
        .iter()
        .enumerate()
        .filter(|e| is_valid[e.0])
        .map(|e| {
            // println!("Update {:?} is valid", e.1);
            let mid = e.1.len() / 2;
            e.1[mid]
        })
        .sum();

    println!("Part 1: {:?}", count);
}

fn part_2(rules_map: &HashMap<i32, HashSet<i32>>, updates: &[Vec<i32>]) {
    let is_valid: Vec<bool> = updates
        .iter()
        .map(|update| {
            update.is_sorted_by(|a, b| {
                rules_map.get(a).is_none_or(|rules| rules.contains(b))
                    && rules_map.get(b).is_none_or(|rules| !rules.contains(a))
            })
        })
        .collect();

    let count: i32 = updates
        .iter()
        .enumerate()
        .filter(|e| !is_valid[e.0])
        .map(|e| {
            // println!("Update {:?} is valid", e.1);
            let mut sorted_update = e.1.clone();
            sorted_update.sort_by(|a, b| {
                if rules_map.get(a).is_none_or(|rules| rules.contains(b))
                    && rules_map.get(b).is_none_or(|rules| !rules.contains(a))
                {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            let mid = sorted_update.len() / 2;
            sorted_update[mid]
        })
        .sum();

    println!("Part 2: {:?}", count);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("assets/2024/day_5.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let split_index = lines.iter().position(|line| line.is_empty()).unwrap();

    let rules: Vec<Vec<i32>> = lines[..split_index]
        .iter()
        .map(|line| {
            line.split("|")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let updates: Vec<Vec<i32>> = lines[split_index + 1..]
        .iter()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    assert!(
        updates.iter().all(|update| update.len() % 2 == 1),
        "All updates must have an odd number of elements"
    );

    let mut rules_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    rules.iter().for_each(|rule| {
        assert_eq!(rule.len(), 2, "Each rule must have exactly two elements");

        rules_map.entry(rule[0]).or_default().insert(rule[1]);
    });

    part_1(&rules_map, &updates);
    part_2(&rules_map, &updates);

    Ok(())
}
