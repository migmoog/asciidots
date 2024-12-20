use std::ops::{Add, AddAssign};

pub struct Point {
    x: u32,
    y: u32
}

#[derive(Copy, Clone)]
pub enum Direction {
    North, // Never 
    East, // Eat
    South, // Soggy
    West, // Waffles :-)
}

impl Add<Direction> for Point {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Point { x: self.x , y: self.y - 1},
            Direction::East => Point { x: self.x + 1, y: self.y },
            Direction::South => Point { x: self.x, y: self.y + 1 },
            Direction::West => Point { x: self.x - 1, y: self.y }
        }
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        *self = Point { x: self.x, y: self.y } + rhs;
    }
}

/**
 * Any type of symbol on a grid that can consume a dot
 */
pub enum Receiver {
    // *
    Asterix,
    // &
    Ampersand,
    // #
    Hash,
    // @
    At,
    // $
    Dollar,
    // ~
    Tilde,
    // [<operator>]
    SquareBrackets,
    // {<operator>} 
    CurlyBrackets,
}