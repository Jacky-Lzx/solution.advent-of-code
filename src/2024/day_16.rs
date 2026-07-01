use advent_of_code::utils::pos::{DIRECTIONS, Direction, Pos, in_bound};
use anyhow::Result;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Element {
    Start,
    End,
    Empty,
    Wall,
}

#[derive(Clone, Copy, Debug)]
struct State {
    pos: Pos,
    dir: Direction,
    point: u32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.dir == other.dir && self.point == other.point
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // self.point.cmp(&other.point)
        other.point.cmp(&self.point) // Reverse order for min-heap behavior
    }
}

impl Eq for State {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct VisitedState {
    pos: Pos,
    dir: Direction,
}

fn part_1(map: &[Vec<Element>]) -> Result<()> {
    let (x_len, y_len) = (map.len(), map[0].len());

    let mut start_pos = None;
    for x in map.iter().enumerate() {
        for y in x.1.iter().enumerate() {
            if &Element::Start == y.1 {
                start_pos = Some(Pos {
                    x: x.0 as i32,
                    y: y.0 as i32,
                });
                break;
            }
        }
    }
    let start_pos = start_pos.expect("Start position not found in map");

    let mut visited = HashSet::<VisitedState>::new();
    let mut frontier = BinaryHeap::<State>::new();

    frontier.push(State {
        pos: start_pos,
        dir: Direction::Right,
        point: 0,
    });

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        visited.insert(VisitedState {
            pos: current.pos,
            dir: current.dir,
        });

        if map[current.pos.x as usize][current.pos.y as usize] == Element::End {
            println!("Part 1: {}", current.point);
            return Ok(());
        }

        for dir in DIRECTIONS {
            let next_pos = current.pos + dir.to_pos();
            if !in_bound(&next_pos, (x_len, y_len)) {
                continue;
            }

            if visited.contains(&VisitedState { pos: next_pos, dir }) {
                continue;
            }

            match map[next_pos.x as usize][next_pos.y as usize] {
                Element::Wall => continue,
                Element::End | Element::Empty => {
                    frontier.push(State {
                        pos: next_pos,
                        dir,
                        point: current.point + if dir != current.dir { 1001 } else { 1 },
                    });
                }
                _ => {}
            }
        }
    }

    Err(anyhow::anyhow!("No path found from start to end"))
}

fn dijkstra(
    map: &[Vec<Element>],
    start_pos: Pos,
    directions: &[Direction],
    reverse: bool,
) -> HashMap<VisitedState, u32> {
    let mut points_map = HashMap::<VisitedState, u32>::new();

    let mut frontier = BinaryHeap::<State>::new();
    for &dir in directions {
        frontier.push(State {
            pos: start_pos,
            dir,
            point: 0,
        });
    }

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if points_map
            .get(&VisitedState {
                pos: current.pos,
                dir: current.dir,
            })
            .is_some_and(|&p| p < current.point)
        {
            continue;
        } else {
            points_map.insert(
                VisitedState {
                    pos: current.pos,
                    dir: current.dir,
                },
                current.point,
            );
        }

        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if dir == current.dir {
                continue;
            }

            frontier.push(State {
                pos: current.pos,
                dir,
                point: current.point + 1000,
            });
        }

        let next_pos = current.pos
            + match current.dir {
                Direction::Up => Pos { x: -1, y: 0 },
                Direction::Down => Pos { x: 1, y: 0 },
                Direction::Left => Pos { x: 0, y: -1 },
                Direction::Right => Pos { x: 0, y: 1 },
            };

        if !in_bound(&next_pos, (map.len(), map[0].len())) {
            continue;
        }

        match map[next_pos.x as usize][next_pos.y as usize] {
            Element::Wall => continue,
            Element::Empty => {
                frontier.push(State {
                    pos: next_pos,
                    dir: current.dir,
                    point: current.point + 1,
                });
            }
            Element::Start if reverse => {
                frontier.push(State {
                    pos: next_pos,
                    dir: current.dir,
                    point: current.point + 1,
                });
            }
            Element::End if !reverse => {
                frontier.push(State {
                    pos: next_pos,
                    dir: current.dir,
                    point: current.point + 1,
                });
            }
            _ => {}
        }
    }

    points_map
}

fn part_2(map: &[Vec<Element>]) -> Result<()> {
    let mut start_pos = None;
    for x in map.iter().enumerate() {
        for y in x.1.iter().enumerate() {
            if &Element::Start == y.1 {
                start_pos = Some(Pos {
                    x: x.0 as i32,
                    y: y.0 as i32,
                });
                break;
            }
        }
    }
    let start_pos = start_pos.expect("start position not found in map");

    let end_pos = map
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().filter_map(move |(y, &element)| {
                if element == Element::End {
                    Some(Pos {
                        x: x as i32,
                        y: y as i32,
                    })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<Pos>>()[0];

    let points_map = dijkstra(map, start_pos, &[Direction::Right], false);

    let mut min_point = u32::MAX;
    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        if let Some(point) = points_map.get(&VisitedState { pos: end_pos, dir }) {
            min_point = min_point.min(*point);
        }
    }

    let reverse_points_map = dijkstra(
        map,
        end_pos,
        &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ],
        true,
    );

    let mut path_points = HashSet::<Pos>::new();

    for (visited_state, &point) in &points_map {
        if reverse_points_map
            .get(&VisitedState {
                pos: visited_state.pos,
                dir: match visited_state.dir {
                    // Reverse the direction for the reverse points map
                    Direction::Up => Direction::Down,
                    Direction::Down => Direction::Up,
                    Direction::Left => Direction::Right,
                    Direction::Right => Direction::Left,
                },
            })
            .is_some_and(|&reverse_point| reverse_point + point == min_point)
        {
            path_points.insert(visited_state.pos);
        }
    }

    // print the map with the path points marked
    // for x in 0..x_len {
    //     for y in 0..y_len {
    //         let pos = Pos {
    //             x: x as i32,
    //             y: y as i32,
    //         };
    //         if path_points.contains(&pos) {
    //             print!("O");
    //         } else {
    //             match map[x][y] {
    //                 Element::Start => print!("S"),
    //                 Element::End => print!("E"),
    //                 Element::Empty => print!("."),
    //                 Element::Wall => print!("#"),
    //             }
    //         }
    //     }
    //     println!();
    // }

    println!("Part 2: {:?}", path_points.len());

    Ok(())
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_16.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let lines: Vec<&str> = contents.lines().collect();

    let map: Vec<Vec<Element>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Element::Start,
                    'E' => Element::End,
                    '.' => Element::Empty,
                    '#' => Element::Wall,
                    _ => panic!("Unexpected character in map: {}", c),
                })
                .collect()
        })
        .collect();

    part_1(&map)?;
    part_2(&map)?;

    Ok(())
}
