#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
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

pub const DIRECTIONS: [Pos; 4] = [
    Pos { x: 0, y: -1 }, // Up
    Pos { x: 1, y: 0 },  // Right
    Pos { x: -1, y: 0 }, // Left
    Pos { x: 0, y: 1 },  // Down
];
