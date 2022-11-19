use crate::{
    dot::{Direction, Dot, Point, Status},
    dot_receivers::DotReceiver,
    map::{string_to_matrix, symbol_within_quote},
};
use std::cmp::{max, min};

pub struct Grid {
    pub dots: Vec<Dot>,
    pub ascii_art: Vec<Vec<char>>,
    pub running: bool,
}
impl Grid {
    pub fn parse(s: String) -> Self {
        let mut out = Self {
            dots: Vec::new(),
            ascii_art: string_to_matrix(s),
            running: true,
        };

        for y in 0..out.ascii_art.len() {
            for (x, c) in out.ascii_art[y].clone().iter().enumerate() {
                // TODO: use symbol_within_quote to check that the dots are not within output for helloworld
                let p = Point { x, y };
                if *c != '.' {
                    continue;
                } else if symbol_within_quote(&out.ascii_art, p).0 {
                    println!("Symbol at {:?} is within quotes", p);
                    continue;
                }

                out.setup_dot(p);
            }
        }
        out
    }

    pub fn tick(&mut self) {
        self.running = self.dots.len() > 0;

        let mut i: usize = 0;
        while i < self.dots.len() {
            if let Status::Held = self.dots[i].status {
                self.dots.remove(i);
            } else {
                self.dots[i].advance(&self.ascii_art);
            }
            i += 1;
        }
    }

    pub fn receiver_check<T: DotReceiver>(
        &mut self,
        rec: &mut T,
        pos: Point,
    ) -> Result<(), String> {
        let dot: &mut Dot;
        match self.dots.iter_mut().find(|d| d.position == pos) {
            Some(d) => dot = d,
            None => return Err("No dot has reached receiver".to_string()),
        }

        if let Some(d) = rec.receive_dot(dot) {
            self.dots.push(d);
        }

        Ok(())
    }

    fn setup_dot(&mut self, p: Point) {
        let direction = self.nearest_track(p);
        if let Some(dir) = direction {
            let d = Dot::new(dir, p);
            self.dots.push(d);
        } else {
            println!("WARNING, Dot at ({}, {}) has no track to follow", p.x, p.y);
            return;
        }
    }

    fn nearest_track(&self, p: Point) -> Option<Direction> {
        let above_index = max(0, p.y as i32 - 1) as usize;
        let one_above = self.ascii_art[above_index][p.x];
        if one_above == '|' || one_above == '+' {
            return Some(Direction::Up);
        }
        let below_index = min(self.ascii_art.len() - 1, p.y + 1);
        let one_below = self.ascii_art[below_index][p.x];
        if one_below == '|' || one_below == '+' {
            return Some(Direction::Down);
        }

        let left_index = max(0, p.x as i32 - 1) as usize;
        let one_left = self.ascii_art[p.y][left_index];
        if one_left == '-' || one_left == '+' {
            return Some(Direction::Left);
        }
        let right_index = min(self.ascii_art[p.y].len() - 1, p.x + 1);
        let one_right = self.ascii_art[p.y][right_index];
        if one_right == '-' || one_right == '+' {
            return Some(Direction::Right);
        }

        None
    }
}
