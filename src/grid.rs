use crate::{
    dot::{Direction, Dot, Point, Status},
    dot_receivers::DotReceiver,
    string_to_matrix,
};
use std::cmp::{max, min};

pub struct Grid {
    pub dots: Vec<Dot>,
    pub ascii_art: Vec<Vec<char>>,
    pub running: bool,
}
impl Grid {
    pub fn parse(s: String) -> Self {
        Self {
            dots: Vec::new(),
            ascii_art: string_to_matrix(s),
            running: true,
        }
    }

    pub fn tick(&mut self) {
        self.running = self.dots.len() > 0;
        for i in 0..self.dots.len() {
            if let Status::Held = self.dots[i].status {
                self.dots.remove(i);
            } else {
                self.dots[i].advance();
            }
        }

        todo!()
    }

    // TODO: will be used in main(), and Grid will no longer own receivers, they'll have their own HashMaps with points
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

        let result = rec.receive_dot(dot);
        if result.is_some() {
            self.dots.push(result.unwrap());
        }

        Ok(())
    }

    fn setup_dot(&mut self, p: Point) {
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
        if one_above == &'|' {
            return Some(Direction::Up);
        }
        let below_index = min(self.ascii_art.len() - 1, p.y + 1);
        let one_below = &self.ascii_art[below_index][p.x];
        if one_below == &'|' {
            return Some(Direction::Down);
        }

        let left_index = max(0, p.x as i32 - 1) as usize;
        let one_left = &self.ascii_art[p.y][left_index];
        if one_left == &'-' {
            return Some(Direction::Left);
        }
        let right_index = min(self.ascii_art[p.y].len() - 1, p.x + 1);
        let one_right = &self.ascii_art[p.y][right_index];
        if one_right == &'-' {
            return Some(Direction::Right);
        }

        None
    }

    pub fn surrounding_symbols(
        map: &Vec<Vec<char>>,
        pos: Point,
    ) -> (Option<char>, Option<char>, Option<char>, Option<char>) {
        let up = if pos.y <= 0 {
            None
        } else {
            Some(map[pos.y - 1][pos.x])
        };

        let down = if pos.y >= map.len() - 1 {
            None
        } else {
            Some(map[pos.y + 1][pos.x])
        };

        let left = if pos.x <= 0 {
            None
        } else {
            Some(map[pos.y][pos.x - 1])
        };

        let right = if pos.x >= map[pos.y].len() - 1 {
            None
        } else {
            Some(map[pos.y][pos.x + 1])
        };

        (up, down, left, right)
    }
}
