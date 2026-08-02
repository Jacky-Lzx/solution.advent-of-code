use advent_of_code::utils::pos::{DIRECTIONS, Direction, Pos, in_bound};
/// Keypad
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
///
/// Controller
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
use anyhow::Result;
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ControllerButton {
    Up,
    Down,
    Left,
    Right,
    A,
    Wall,
}

impl ControllerButton {
    pub fn as_pos(&self) -> Pos {
        match self {
            ControllerButton::Up => Pos { x: 0, y: 1 },
            ControllerButton::Down => Pos { x: 1, y: 1 },
            ControllerButton::Left => Pos { x: 1, y: 0 },
            ControllerButton::Right => Pos { x: 1, y: 2 },
            ControllerButton::A => Pos { x: 0, y: 2 },
            ControllerButton::Wall => unreachable!("Wall button has no position"),
        }
    }
}

const KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['#', '0', 'A'],
];

const CONTROLLER: [[char; 3]; 2] = [['#', '^', 'A'], ['<', 'v', '>']];

#[derive(Eq, PartialEq)]
struct HeapState {
    state: State,
    path_len: usize,
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
struct State {
    pos: Pos,
    button: ControllerButton,
}

impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse the order to make it a min-heap
        other.path_len.cmp(&self.path_len)
    }
}

fn find_keyboard_path(
    start: Pos,
    end: Pos,
    max_level: usize,
    dp: &mut HashMap<(usize, ControllerButton, ControllerButton), usize>,
) -> usize {
    let sizes = (KEYPAD.len(), KEYPAD[0].len());

    let mut frontier = BinaryHeap::<HeapState>::new();
    let mut visited = HashMap::<State, usize>::new();

    frontier.push(HeapState {
        state: State {
            pos: start,
            button: ControllerButton::A,
        },
        path_len: 0,
    });

    let mut best_length = usize::MAX;

    while let Some(current) = frontier.pop() {
        visited.insert(current.state, current.path_len);

        // Find a path to the end does not mean it is the shortest path. Cannot directly return the current path
        if current.state.pos == end {
            let result = current.path_len
                + find_controller_path(
                    1,
                    max_level,
                    State {
                        pos: current.state.button.as_pos(),
                        button: ControllerButton::A,
                    },
                    ControllerButton::A.as_pos(),
                    dp,
                );

            best_length = best_length.min(result);

            continue;
        }

        for dir in DIRECTIONS {
            let next_pos = current.state.pos + dir.to_pos();

            if !in_bound(&next_pos, sizes)
                || KEYPAD[next_pos.x as usize][next_pos.y as usize] == '#'
            {
                continue;
            }

            let next_button = dir.into();

            let next_state = State {
                pos: next_pos,
                button: next_button,
            };

            let new_len = find_controller_path(
                1,
                max_level,
                State {
                    pos: current.state.button.as_pos(),
                    button: ControllerButton::A,
                },
                next_button.as_pos(),
                dp,
            );

            let next_path_len = current.path_len + new_len;

            if visited
                .get(&next_state)
                .is_some_and(|path_len| next_path_len >= *path_len)
            {
                continue;
            }

            frontier.push(HeapState {
                state: next_state,
                path_len: next_path_len,
            });
        }
    }

    best_length
}

/// Find a path in the controller
/// Arguments:
/// `level`: the current level of recursion
/// `max_level`: the maximum level of recursion
/// `start`: the start state
///      - pos: start position
///      - button: the last button on the higher level controller
/// `end`: the end position
///
/// If level == max_level, the the path is just the direction from start to end
fn find_controller_path(
    level: usize,
    max_level: usize,
    start: State,
    end: Pos,
    dp: &mut HashMap<(usize, ControllerButton, ControllerButton), usize>,
) -> usize {
    let dp_entry = (level, start.pos.into(), end.into());
    if let Some(&cached_len) = dp.get(&dp_entry) {
        return cached_len;
    }

    let map = CONTROLLER;
    let sizes = (map.len(), map[0].len());

    let mut visited = HashMap::<State, usize>::new();

    let mut frontier = BinaryHeap::<HeapState>::new();
    frontier.push(HeapState {
        state: start,
        path_len: 0,
    });

    let mut best_length = usize::MAX;

    while let Some(current) = frontier.pop() {
        visited.insert(current.state, current.path_len);

        // Find a path to the end does not mean it is the shortest path. Cannot directly return the current path
        if current.state.pos == end {
            let mut result_len = current.path_len;
            if level < max_level {
                result_len += find_controller_path(
                    level + 1,
                    max_level,
                    State {
                        pos: current.state.button.as_pos(),
                        button: ControllerButton::A,
                    },
                    ControllerButton::A.as_pos(),
                    dp,
                )
            } else {
                // result_path += "A";
                result_len += 1;
            }

            best_length = best_length.min(result_len);
            continue;
        }

        for dir in DIRECTIONS {
            let next_pos = current.state.pos + dir.to_pos();

            if !in_bound(&next_pos, sizes)
                || ControllerButton::from(map[next_pos.x as usize][next_pos.y as usize])
                    == ControllerButton::Wall
            {
                continue;
            }

            let next_button: ControllerButton = dir.into();

            let next_path_len = current.path_len
                + if level == max_level {
                    // ControllerButton::from(dir).to_string()
                    1
                } else {
                    find_controller_path(
                        level + 1,
                        max_level,
                        State {
                            pos: current.state.button.as_pos(),
                            button: ControllerButton::A,
                        },
                        next_button.as_pos(),
                        dp,
                    )
                };

            let next_state = State {
                pos: next_pos,
                button: next_button,
            };

            if visited
                .get(&next_state)
                .is_some_and(|path_len| next_path_len >= *path_len)
            {
                continue;
            }

            frontier.push(HeapState {
                state: next_state,
                path_len: next_path_len,
            });
        }
    }

    dp.insert(dp_entry, best_length);

    best_length
}

fn solve_single(sequence: &str, max_level: usize) -> usize {
    let sequence = "A".to_string() + sequence;

    let result = sequence
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .fold(0, |result, pair| {
            println!("{} -> {}", pair[0], pair[1]);

            let start_pos = get_keypad_pos(pair[0]);
            let end_pos = get_keypad_pos(pair[1]);

            result + find_keyboard_path(start_pos, end_pos, max_level, &mut HashMap::new())
        });

    println!("Sequence: {}, Result: {}", sequence, result);

    result
}

fn solve(sequences: &[String], max_level: usize) -> usize {
    sequences
        .iter()
        .map(|seq| {
            solve_single(seq, max_level)
                * seq[..seq.len() - 1]
                    .parse::<usize>()
                    .expect("Parse sequence to u32 failed")
        })
        .sum()
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("assets/2024/day_21.input")?;
    // let contents = fs::read_to_string("assets/test.input")?;
    // println!("File contents:\n{}", contents);

    let sequences = contents
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>();

    println!("Part 1: {}", solve(&sequences, 2));
    println!("Part 2: {}", solve(&sequences, 25));

    Ok(())
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {})",
            match self.pos {
                Pos { x: 0, y: 1 } => "^",
                Pos { x: 0, y: 2 } => "A",
                Pos { x: 1, y: 0 } => "<",
                Pos { x: 1, y: 1 } => "v",
                Pos { x: 1, y: 2 } => ">",
                _ => unreachable!("Invalid position: {:?}", self.pos),
            },
            self.button
        )
    }
}

impl Display for ControllerButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            ControllerButton::Up => '^',
            ControllerButton::Down => 'v',
            ControllerButton::Left => '<',
            ControllerButton::Right => '>',
            ControllerButton::A => 'A',
            ControllerButton::Wall => '#',
        };
        write!(f, "{}", c)
    }
}

impl From<Direction> for ControllerButton {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => ControllerButton::Up,
            Direction::Down => ControllerButton::Down,
            Direction::Left => ControllerButton::Left,
            Direction::Right => ControllerButton::Right,
        }
    }
}

impl From<Pos> for ControllerButton {
    fn from(pos: Pos) -> Self {
        match pos {
            Pos { x: 0, y: 1 } => ControllerButton::Up,
            Pos { x: 0, y: 2 } => ControllerButton::A,
            Pos { x: 1, y: 0 } => ControllerButton::Left,
            Pos { x: 1, y: 1 } => ControllerButton::Down,
            Pos { x: 1, y: 2 } => ControllerButton::Right,
            _ => unreachable!("Invalid position: {:?}", pos),
        }
    }
}

impl From<char> for ControllerButton {
    fn from(c: char) -> Self {
        match c {
            '^' => ControllerButton::Up,
            'v' => ControllerButton::Down,
            '<' => ControllerButton::Left,
            '>' => ControllerButton::Right,
            'A' => ControllerButton::A,
            '#' => ControllerButton::Wall,
            _ => panic!("Invalid controller button: {}", c),
        }
    }
}

fn get_keypad_pos(c: char) -> Pos {
    for (i, row) in KEYPAD.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == c {
                return Pos {
                    x: i as i32,
                    y: j as i32,
                };
            }
        }
    }
    unreachable!("Character {} not found in keypad", c);
}
