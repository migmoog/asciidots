use crate::{
    dot::{self, Direction, Dot, Point},
    dot_receivers::DotReceiver,
    tokens::TRACKS,
};
use std::cmp::{max, min};

pub struct Grid {
    receivers: Vec<Box<dyn DotReceiver>>,
    dots: Vec<Dot>,
    ascii_art: Vec<Vec<char>>,
    running: bool,
}
impl Grid {
    pub fn parse(s: String) -> Self {
        println!("{s}");

        // FIXME: more than one line destroys this
        let mut out = Self {
            // TODO: implement all receivers
            receivers: Vec::new(),
            dots: Vec::new(),
            ascii_art: Vec::new(),
            running: true,
        };

        for line in s.trim_end().split('\n') {
            let mut l_vec = Vec::new();
            for c in line.chars() {
                l_vec.push(c);
            }
            out.ascii_art.push(l_vec);
        }

        out.ascii_art.iter().for_each(|v| println!("{:#?}", v));

        for (y, line) in out.ascii_art.clone().iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if c != &'.' {
                    continue;
                }

                out.add_dot(Point { x, y });
            }
        }
        out.dots.iter().for_each(|d| println!("{:?}", d));

        out
    }

    pub fn tick(&mut self) {
        for y in 0..self.ascii_art.len() {
            for x in 0..self.ascii_art[y].len() {
                let pos = dot::Point { x, y };
                // TODO, locate operator at point
            }
        }

        for d in self.dots.iter_mut() {
            d.advance();
        }
    }

    fn add_dot(&mut self, p: Point) {
        let direction = self.nearest_track(&p);
        if direction.is_none() {
            println!("WARNING, Dot at ({}, {}) has no track to follow", p.x, p.y);
            return;
        }

        let d = Dot::new(direction.unwrap(), p);
        self.dots.push(d);
    }

    fn nearest_track(&self, p: &Point) -> Option<Direction> {
        let above_index = max(0, p.y as i32 - 1) as usize;
        let one_above = &self.ascii_art[above_index][p.x];
        if TRACKS.contains(one_above) && one_above == &'|' {
            return Some(Direction::Up);
        }
        let below_index = min(self.ascii_art.len() - 1, p.y + 1);
        let one_below = &self.ascii_art[below_index][p.x];
        if TRACKS.contains(one_below) && one_below == &'|' {
            return Some(Direction::Down);
        }

        let left_index = max(0, p.x as i32 - 1) as usize;
        let one_left = &self.ascii_art[p.y][left_index];
        if TRACKS.contains(one_left) && one_left == &'-' {
            return Some(Direction::Left);
        }
        let right_index = min(self.ascii_art[p.y].len() - 1, p.x + 1);
        let one_right = &self.ascii_art[p.y][right_index];
        if TRACKS.contains(one_right) && one_right == &'-' {
            return Some(Direction::Right);
        }

        None
    }
}
