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

    pub fn rotate(&self, clockwise: bool) -> Self {
        match self {
            &Self::Down => {
                if clockwise {
                    Self::Left
                } else {
                    Self::Right
                }
            }
            &Self::Left => {
                if clockwise {
                    Self::Up
                } else {
                    Self::Down
                }
            }
            &Self::Up => {
                if clockwise {
                    Self::Right
                } else {
                    Self::Left
                }
            }
            &Self::Right => {
                if clockwise {
                    Self::Down
                } else {
                    Self::Up
                }
            }
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

    pub fn advance(&mut self, map: &Vec<Vec<char>>) {
        if map[self.position.y][self.position.x] == ' ' {
            self.status = Status::Held;
        }
        if let Status::Held = self.status {
            return;
        }

        match self.dir {
            Direction::Down => {
                if self.position.y == map.len() - 1 {
                    self.status = Status::Held;
                } else {
                    self.position.y += 1;
                }
            }
            Direction::Up => {
                if self.position.y == 0 {
                    self.status = Status::Held;
                } else {
                    self.position.y -= 1;
                }
            }
            Direction::Left => {
                if self.position.x == 0 {
                    self.status = Status::Held;
                } else {
                    self.position.x -= 1;
                }
            }
            Direction::Right => {
                if self.position.x == map[self.position.y].len() - 1 {
                    self.status = Status::Held;
                } else {
                    self.position.x += 1;
                }
            }
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
