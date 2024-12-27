use std::ops::{Add, AddAssign};

use crate::dot::Dot;

#[derive(Clone, PartialEq, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North, // Never
    East,  // Eat
    South, // Soggy
    West,  // Waffles :-)
}

impl Add<Direction> for Point {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        let new = self.clone() + rhs;
        self.x = new.x;
        self.y = new.y;
    }
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        let (x, y) = value;
        Self { x, y }
    }
}


/**
 * Any type of symbol on a grid that can consume a dot
 */
pub enum ReceiverNode {
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

pub enum CouldntReceive {
    DotAtAmpersand
}

pub trait Receiver {
    fn receive(&mut self, d: Dot) -> Result<Vec<Dot>, CouldntReceive>;
}

impl Receiver for ReceiverNode {
    fn receive(&mut self, d: Dot) -> Result<Vec<Dot>, CouldntReceive> {
        use ReceiverNode::*;
        match *self {
            Ampersand => Err(CouldntReceive::DotAtAmpersand),
            _ => todo!()
        }
    }
}