use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part_1(_connections: &[[&str; 2]], connection_map: &HashMap<&str, HashSet<&str>>) -> Result<()> {
    let mut answer = HashSet::<String>::new();

    for (k, connects) in connection_map.iter() {
        if !k.starts_with('t') {
            continue;
        }

        for c in connects {
            for c_connect in connection_map.get(c).unwrap() {
                if connects.contains(c_connect) {
                    // println!("{}-{}-{}", k, c, c_connect);
                    let mut vec = [k.to_string(), c.to_string(), c_connect.to_string()];
                    vec.sort();

                    let s = vec.join("-");
                    answer.insert(s);
                }
            }
        }
    }

    println!("Part 1: {}", answer.len());

    Ok(())
}

#[allow(dead_code)]
fn naive<'a>(
    connections: &[[&'a str; 2]],
    connection_map: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    let mut fully_connected_sets: Vec<HashSet<&str>> = Vec::new();

    for c in connections {
        let mut connected_set = HashSet::new();
        connected_set.insert(c[0]);
        connected_set.insert(c[1]);

        fully_connected_sets.push(connected_set);
    }

    for computer in connection_map.keys() {
        let mut additional_sets = Vec::new();
        for sets in &fully_connected_sets {
            if sets.contains(computer) {
                continue;
            }

            if sets
                .iter()
                .all(|c| connection_map.get(computer).unwrap().contains(c))
            {
                let mut new_set = sets.clone();
                new_set.insert(computer);
                additional_sets.push(new_set);
            }
        }

        fully_connected_sets.extend(additional_sets);
    }

    fully_connected_sets.sort_by_key(|s| s.len());

    fully_connected_sets.pop().unwrap()
}

fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    p: HashSet<&'a str>,
    x: HashSet<&'a str>,
    connection_map: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    if p.is_empty() && x.is_empty() {
        return r;
    }

    let mut results: Option<HashSet<_>> = None;

    let mut new_p = p.clone();
    let mut new_x = x.clone();

    for v in p.iter() {
        new_p.insert(v);

        let neighbors = connection_map.get(v).unwrap();

        let next_p = new_p
            .intersection(neighbors)
            .copied()
            .collect::<HashSet<_>>();
        let next_x = x.intersection(neighbors).copied().collect::<HashSet<_>>();

        let mut new_r = r.clone();
        new_r.insert(v);

        let ret = bron_kerbosch(new_r, next_p, next_x, connection_map);

        if results.is_none() || ret.len() > results.as_ref().unwrap().len() {
            results = Some(ret);
        }

        new_p.remove(v);
        new_x.insert(v);
    }

    results.expect("bron_kerbosch should always return a result if p is non-empty")
}

#[allow(unused_variables)]
fn part_2(connections: &[[&str; 2]], connection_map: &HashMap<&str, HashSet<&str>>) -> Result<()> {
    // let result = naive(connections, connection_map);

    let result = bron_kerbosch(
        HashSet::new(),
        connection_map.keys().copied().collect(),
        HashSet::new(),
        connection_map,
    );

    let mut result = result.iter().copied().collect::<Vec<_>>();
    result.sort();

    let result = result.join(",");

    println!("Part 2: {:?}", result);

    Ok(())
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_23.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let connections = contents
        .lines()
        .map(|line| {
            let mut s = line.trim().split('-');
            [
                s.next().expect("Expect first split"),
                s.next().expect("Expect second split"),
            ]
        })
        .collect::<Vec<_>>();

    let mut connection_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for c in &connections {
        connection_map
            .entry(c[0])
            .and_modify(|e| {
                e.insert(c[1]);
            })
            .or_insert(HashSet::from([c[1]]));

        connection_map
            .entry(c[1])
            .and_modify(|e| {
                e.insert(c[0]);
            })
            .or_insert(HashSet::from([c[0]]));
    }

    part_1(&connections, &connection_map)?;
    part_2(&connections, &connection_map)?;

    Ok(())
}
