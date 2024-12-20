use crate::grid::{Direction, LoopMember, Point};

pub struct Dot {
    pos: Point,
    value: f64,
    trajectory: Direction 
}

impl LoopMember for Dot {
    // advances the dot along the track it's currently on
    fn tick(&mut self) {
        self.pos += self.trajectory;
    }
}