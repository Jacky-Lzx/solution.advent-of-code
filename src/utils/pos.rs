#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Mul<i32> for Pos {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    pub fn to_pos(&self) -> Pos {
        match self {
            Direction::Up => Pos { x: -1, y: 0 },
            Direction::Down => Pos { x: 1, y: 0 },
            Direction::Left => Pos { x: 0, y: -1 },
            Direction::Right => Pos { x: 0, y: 1 },
        }
    }
}

impl Pos {
    pub fn to_direction(&self) -> Option<Direction> {
        match (self.x / self.x.abs(), self.y / self.y.abs()) {
            (-1, 0) => Some(Direction::Up),
            (1, 0) => Some(Direction::Down),
            (0, -1) => Some(Direction::Left),
            (0, 1) => Some(Direction::Right),
            _ => None,
        }
    }
}

pub fn in_bound(pos: &Pos, sizes: (usize, usize)) -> bool {
    pos.x >= 0 && pos.x < sizes.0 as i32 && pos.y >= 0 && pos.y < sizes.1 as i32
}
