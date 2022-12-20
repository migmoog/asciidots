use std::collections::HashMap;

use crate::{
    dot::{Direction, Dot, Point, Status},
    map::{get_quote, surrounding_symbols},
};

/**
For any object on the grid that accepts dots and need to return another.
 */
pub trait DotReceiver {
    fn receive_dot(&mut self, _dot: &mut Dot) -> Option<Dot> {
        None
    }
    fn get_character(&self) -> char;
}

type RecGrid<R> = HashMap<Point, R>;

// TODO implement all receivers and add them to this tuple struct
pub struct Receivers(
    pub RecGrid<Operation>,
    pub RecGrid<Ampersand>,
    pub RecGrid<Dollar>,
    pub RecGrid<Slash>,
);

impl Receivers {
    pub fn new(map: &Vec<Vec<char>>) -> Self {
        let mut out = Self(
            RecGrid::new(),
            RecGrid::new(),
            RecGrid::new(),
            RecGrid::new(),
        );

        for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                let p = Point { x, y };

                match *c {
                    '{' | '[' => {
                        let op = map[y][x + 1];
                        let b2 = map[y][x + 2];

                        out.0
                            .insert(p, Operation::new(Axis::from(*c, b2).unwrap(), op));
                    }
                    '&' => {
                        out.1.insert(p, Ampersand);
                    }
                    '$' => {
                        let (direction, message) = Dollar::get_message(map, p);

                        out.2.insert(p, Dollar { message, direction });
                    }
                    '/' | '\\' => {
                        out.3.insert(
                            p,
                            Slash {
                                backslash: *c == '\\',
                            },
                        );
                    }
                    _ => {}
                }
            }
        }

        out
    }
}

pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn from(b1: char, b2: char) -> Result<Self, String> {
        const MSG: &str = "Differing brackets at ({}, {})! Must be the same for an operation";
        if b2 as u32 != b1 as u32 + 1 {
            return Err(MSG.to_string());
        }

        match b1 {
            '{' => Ok(Self::X),
            '[' => Ok(Self::Y),
            _ => Err(MSG.to_string()),
        }
    }
}

pub struct Operation {
    axis: Axis,
    horiz_dot: Option<Dot>,
    vert_dot: Option<Dot>,
    op: char,
}

impl Operation {
    pub fn new(axis: Axis, op: char) -> Self {
        Self {
            axis,
            op,
            horiz_dot: None,
            vert_dot: None,
        }
    }
}

impl DotReceiver for Operation {
    fn receive_dot(&mut self, dot: &mut Dot) -> Option<Dot> {
        let is_vert = dot.dir.is_vertical();
        if is_vert {
            if self.vert_dot.is_none() {
                dot.status = Status::Held;
                self.vert_dot = Some(dot.clone());

                return None;
            }
        } else {
            if self.horiz_dot.is_none() {
                dot.status = Status::Held;
                self.horiz_dot = Some(dot.clone());

                return None;
            }
        }

        if self.horiz_dot.is_none() || self.vert_dot.is_none() {
            return None;
        }

        let x_dot = self.horiz_dot.as_mut().unwrap();
        let y_dot = self.vert_dot.as_mut().unwrap();

        let direction = match self.axis {
            Axis::X => x_dot.dir.clone(),
            Axis::Y => y_dot.dir.clone(),
        };
        let position = match direction {
            Direction::Down => Point {
                x: dot.position.x,
                y: dot.position.y + 1,
            },
            Direction::Up => Point {
                x: dot.position.x,
                y: dot.position.y - 1,
            },
            Direction::Right => Point {
                x: dot.position.x + 1,
                y: dot.position.y,
            },
            Direction::Left => Point {
                x: dot.position.x - 1,
                y: dot.position.y,
            },
        };

        let mut out = Dot::new(direction, position);

        let xv = x_dot.value;
        let yv = y_dot.value;
        // this whole thing is utterly evil but whatever
        out.value = match self.op {
            '*' => xv * yv,
            // Axis matching is required for non-commutative operations
            '/' => match self.axis {
                Axis::X => xv / yv,
                Axis::Y => yv / xv,
            },
            '+' => xv + yv,
            '-' => match self.axis {
                Axis::X => xv - yv,
                Axis::Y => yv - xv,
            },
            '%' => match self.axis {
                Axis::X => xv % yv,
                Axis::Y => yv % xv,
            },
            '^' => match self.axis {
                Axis::X => xv.powf(yv),
                Axis::Y => yv.powf(xv),
            },
            '&' => ((xv > 0.0 && yv > 0.0) as i32) as f64,
            'o' => ((xv > 0.0 || yv > 0.0) as i32) as f64,
            // TODO: XOR operator
            '>' => {
                ((match self.axis {
                    Axis::X => xv > yv,
                    Axis::Y => yv > xv,
                }) as i32) as f64
            }
            'G' => {
                ((match self.axis {
                    Axis::X => xv >= yv,
                    Axis::Y => yv >= xv,
                }) as i32) as f64
            }
            '<' => {
                ((match self.axis {
                    Axis::X => xv < yv,
                    Axis::Y => yv < xv,
                }) as i32) as f64
            }
            'L' => {
                ((match self.axis {
                    Axis::X => xv <= yv,
                    Axis::Y => yv <= xv,
                }) as i32) as f64
            }
            '=' => ((xv == yv) as i32) as f64,
            '!' => ((xv != yv) as i32) as f64,
            c => panic!(
                "Error at position ({},{}), {} is not a valid operation",
                dot.position.x, dot.position.y, c
            ),
        };

        Some(out)
    }

    fn get_character(&self) -> char {
        match self.axis {
            Axis::X => '{',
            Axis::Y => '[',
        }
    }
}

pub struct Ampersand;

impl DotReceiver for Ampersand {
    fn get_character(&self) -> char {
        '&'
    }
}

#[derive(Debug)]
pub struct Dollar {
    message: String,
    direction: Direction,
}

impl Dollar {
    fn get_message(map: &Vec<Vec<char>>, sign_pos: Point) -> (Direction, String) {
        match surrounding_symbols(map, sign_pos) {
            // above character is a quote with a track below
            [Some(('"', p)), Some(('|', _)), ..] => {
                (Direction::Up, get_quote(map, p, Direction::Up))
            }
            // below character is a quote with a track above
            [Some(('|', _)), Some(('"', p)), ..] => {
                (Direction::Down, get_quote(map, p, Direction::Down))
            }
            // and the same thing horizontally
            [_, _, Some(('-', _)), Some(('"', p))] => {
                (Direction::Right, get_quote(map, p, Direction::Right))
            }
            [_, _, Some(('"', p)), Some(('-', _))] => {
                (Direction::Left, get_quote(map, p, Direction::Left))
            }

            _ => unreachable!(),
        }
    }
}

impl DotReceiver for Dollar {
    fn receive_dot(&mut self, dot: &mut Dot) -> Option<Dot> {
        println!("{}", self.message);

        if self.direction == dot.dir {
            let l = self.message.len();
            match self.direction {
                Direction::Up => {
                    dot.position.y -= l;
                }
                Direction::Down => {
                    dot.position.y += l;
                }
                Direction::Left => {
                    dot.position.x -= l;
                }
                Direction::Right => {
                    dot.position.x += l;
                }
            }
        }
        None
    }

    fn get_character(&self) -> char {
        '$'
    }
}

pub struct Slash {
    pub backslash: bool,
}

impl DotReceiver for Slash {
    fn receive_dot(&mut self, dot: &mut Dot) -> Option<Dot> {
        // this is refactorable but I just want it to work
        dot.dir = if !self.backslash {
            match dot.dir {
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Up => Direction::Right,
            }
        } else {
            match dot.dir {
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Up => Direction::Left,
            }
        };
        // println!("Bounced dot {:?}", dot);

        None
    }

    fn get_character(&self) -> char {
        if self.backslash {
            '\\'
        } else {
            '/'
        }
    }
}

pub struct Hashtag {
    value: f64,
    set: usize,
    dir: Direction,
}

impl Hashtag {
    fn next_to_dollar(map: &Vec<Vec<char>>, pos: Point) -> bool {
        for sym_pair in surrounding_symbols(map, pos) {
            if let Some(('$', _)) = sym_pair {
                return true;
            }
        }

        false
    }
}

impl DotReceiver for Hashtag {
    fn receive_dot(&mut self, dot: &mut Dot) -> Option<Dot> {
        dot.value = self.value;

        None
    }

    fn get_character(&self) -> char {
        '#'
    }
}
