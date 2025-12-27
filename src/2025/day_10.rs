use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    fs::read_to_string,
};

type Machine = (Vec<bool>, Vec<Vec<usize>>, Vec<i32>);

#[derive(Hash, PartialEq, Debug, Clone, Eq)]
struct Lights(Vec<bool>);

#[derive(Hash, PartialEq, Debug, Clone, Eq)]
struct Joltage(Vec<i32>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct HeapItem {
    estimated_cost: i32,
    cost: i32,
    joltage: Joltage,
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimated_cost.cmp(&self.estimated_cost)
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs_part_1(machine: &Machine) -> Option<i32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let initial_lights = Lights(vec![false; machine.0.len()]);
    queue.push_back((initial_lights.clone(), 0));
    visited.insert(initial_lights);

    while let Some((state, cost)) = queue.pop_front() {
        if state.0 == machine.0 {
            return Some(cost);
        }

        for action in &machine.1 {
            let mut new_state = state.clone();
            for &idx in action {
                new_state.0[idx] = !new_state.0[idx];
            }

            if !visited.contains(&new_state) {
                visited.insert(new_state.clone());
                queue.push_back((new_state, cost + 1));
            }
        }
    }

    None
}

fn estimate_cost(joltage: &Joltage, target: &[i32]) -> i32 {
    let mut max_diff = 0;
    for (i, &j) in joltage.0.iter().enumerate() {
        max_diff = max_diff.max(target[i] - j);
    }
    max_diff
}

fn a_star_part_2(machine: &Machine) -> Option<i32> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    let initial_joltage = Joltage(vec![0; machine.2.len()]);
    let initial_estimated = estimate_cost(&initial_joltage, &machine.2);
    heap.push(HeapItem {
        estimated_cost: initial_estimated,
        cost: 0,
        joltage: initial_joltage.clone(),
    });
    visited.insert(initial_joltage);

    while let Some(item) = heap.pop() {
        if item.joltage.0 == machine.2 {
            return Some(item.cost);
        }

        for action in &machine.1 {
            let mut new_joltage = item.joltage.clone();
            let mut valid = true;

            for &idx in action {
                new_joltage.0[idx] += 1;
                if new_joltage.0[idx] > machine.2[idx] {
                    valid = false;
                    break;
                }
            }

            if !valid {
                continue;
            }

            if !visited.contains(&new_joltage) {
                visited.insert(new_joltage.clone());
                let new_cost = item.cost + 1;
                let estimated = new_cost + estimate_cost(&new_joltage, &machine.2);
                heap.push(HeapItem {
                    estimated_cost: estimated,
                    cost: new_cost,
                    joltage: new_joltage,
                });
            }
        }
    }

    None
}

fn part_1(machines: &[Machine]) {
    let total: i32 = machines.iter().map(|m| bfs_part_1(m).unwrap()).sum();
    println!("Part 1: {}", total);
}

fn part_2(machines: &[Machine]) {
    let result: i32 = machines
        .iter()
        .enumerate()
        .map(|(i, machine)| {
            println!("Machine: {}", i);
            a_star_part_2(machine).unwrap()
        })
        .sum();

    println!("Part 2: {}", result);
}

fn main() {
    let contents = read_to_string("assets/2025/day_10.input").unwrap();
    // let contents = read_to_string("assets/2025/test.input").unwrap();

    let machines = contents
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let len = parts.len();

            let diagram = parts[0]
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .chars()
                .map(|c| c == '#')
                .collect();

            let buttons = parts[1..len - 1]
                .iter()
                .map(|x| {
                    x.strip_prefix('(')
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .split(',')
                        .map(|y| y.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();

            let joltage = parts[len - 1]
                .strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();

            (diagram, buttons, joltage)
        })
        .collect::<Vec<Machine>>();

    part_1(&machines);
    part_2(&machines);
}
