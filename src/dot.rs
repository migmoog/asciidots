#[derive(Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    pub fn is_vertical(&self) -> bool {
        match self {
            &Self::Left | &Self::Right => false,
            &Self::Up | &Self::Down => true,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    Held,
    Moving,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    const ZERO: Self = Self { x: 0, y: 0 };
}

#[derive(Clone, Debug)]
pub struct Dot {
    pub value: f64,
    pub dir: Direction,
    pub status: Status,
    pub position: Point,
}
impl Dot {
    pub fn new(dir: Direction, position: Point) -> Self {
        Self {
            dir,
            position,
            ..Default::default()
        }
    }

    pub fn advance(&mut self) {
        if self.status != Status::Moving {
            return;
        }

        match self.dir {
            Direction::Down => self.position.y += 1,
            Direction::Up => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }
}

impl Default for Dot {
    fn default() -> Self {
        Self {
            value: 0.0,
            dir: Direction::Right,
            status: Status::Moving,
            position: Point::ZERO,
        }
    }
}
