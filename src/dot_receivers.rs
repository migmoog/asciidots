use crate::dot::{Direction, Dot, Point, Status};

/**
    For any object on the grid that accepts dots and need to return another.
*/
pub trait DotReceiver {
    fn receive_dot(&mut self, dot: &mut Dot) -> Option<Dot>;
}

// TODO: implement DotReceiver
pub enum Axis {
    X,
    Y,
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
}

struct Ampersand;
impl DotReceiver for Ampersand {
    fn receive_dot(&mut self, dot: &mut Dot) -> Option<Dot> {
        panic!(
            "Dot reached &, grid position: ({}, {})",
            dot.position.x, dot.position.y
        );
    }
}

struct Dollar {
    message: String,
}
impl DotReceiver for Dollar {
    fn receive_dot(&mut self, _dot: &mut Dot) -> Option<Dot> {
        println!("{}", self.message);
        None
    }
}