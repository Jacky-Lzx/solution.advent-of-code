use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

const NUM: usize = 2000;

struct PseudoRandom {
    seed: usize,
    is_first: bool,
}

impl PseudoRandom {
    fn new(seed: usize) -> Self {
        Self {
            seed,
            is_first: true,
        }
    }
}

impl Iterator for PseudoRandom {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            return Some(self.seed);
        }

        const MODULE: usize = 16777216;

        let mut secret = self.seed;
        let mut next;

        // Step 1
        next = secret * 64;
        next ^= self.seed;
        next %= MODULE;
        secret = next;

        // Step 2
        next = secret / 32;
        next ^= secret;
        next %= MODULE;
        secret = next;

        // Step 3
        next = secret * 2048;
        next ^= secret;
        next %= MODULE;

        self.seed = next;

        Some(next)
    }
}

fn part_1(numbers: &[usize]) -> Result<()> {
    let answer = numbers
        .iter()
        .map(|&n| {
            PseudoRandom::new(n)
                .nth(NUM)
                .expect("Failed to generate number")
        })
        .sum::<usize>();

    println!("Part 1: {}", answer);

    Ok(())
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Sequence(isize, isize, isize, isize);

fn build_hash_map(seq: &[usize]) -> HashMap<Sequence, usize> {
    seq.windows(5).fold(HashMap::new(), |mut map, window| {
        let v = (0..4)
            .map(|i| window[i + 1] as isize - window[i] as isize)
            .collect::<Vec<_>>();

        // The monkey sell the spot when it first see the matching price changes
        map.entry(Sequence(v[0], v[1], v[2], v[3]))
            .or_insert(window[4]);

        map
    })
}

fn part_2(numbers: &[usize]) -> Result<()> {
    let maps = numbers
        .iter()
        .map(|&n| {
            PseudoRandom::new(n)
                .map(|x| x % 10)
                .take(NUM)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let hash_maps = maps
        .iter()
        .map(|seq| build_hash_map(seq))
        .collect::<Vec<_>>();

    let unvisited = hash_maps
        .iter()
        .flat_map(|map| map.keys())
        .collect::<HashSet<_>>();

    let mut best_price = 0;

    unvisited.iter().for_each(|seq| {
        let values = hash_maps
            .iter()
            .map(|map| map.get(seq).copied().unwrap_or(0));

        let num = values.clone().sum::<usize>();

        best_price = best_price.max(num);
    });

    println!("Part 2: {}", best_price);

    Ok(())
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_22.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let numbers = contents
        .lines()
        .map(|line| {
            line.trim()
                .parse::<usize>()
                .expect("Failed to parse line as usize")
        })
        .collect::<Vec<_>>();

    part_1(&numbers)?;
    part_2(&numbers)?;

    Ok(())
}
