use crate::{
    context::Cart,
    grid::{Direction, Point},
};

pub struct Dot {
    pub pos: Point,
    pub value: f64,
    pub trajectory: Direction,
}

impl Cart for Dot {
    type Output = ();
    // advances the dot along the track it's currently on
    fn tick(&mut self) -> Self::Output {
        self.pos += self.trajectory;
    }
}

#[cfg(test)]
mod dot_tests {
    use super::*;
    fn make_dot1(coords: (u32, u32), trajectory: Direction) -> Dot {
        let (x, y) = coords;
        Dot {
            pos: Point { x, y },
            value: 1.0,
            trajectory,
        }
    }

    #[test]
    fn ticks_north() {
        let mut west = make_dot1((0, 3), Direction::North);
        for i in (1..=3).rev() {
            assert_eq!(Point { x: 0, y: i }, west.pos);
            west.tick();
        }
        assert_eq!(Point { x: 0, y: 0 }, west.pos);
    }

    #[test]
    fn ticks_west() {
        let mut west = make_dot1((3, 0), Direction::West);
        for i in (1..=3).rev() {
            assert_eq!(Point { x: i, y: 0 }, west.pos);
            west.tick();
        }
        assert_eq!(Point { x: 0, y: 0 }, west.pos);
    }

    #[test]
    fn ticks_south() {
        let mut east = make_dot1((0, 0), Direction::South);
        for i in 0..3 {
            assert_eq!(Point { x: 0, y: i }, east.pos);
            east.tick();
        }
        assert_eq!(Point { x: 0, y: 3 }, east.pos);
    }

    #[test]
    fn ticks_east() {
        let mut east = make_dot1((0, 0), Direction::East);
        for i in 0..3 {
            assert_eq!(Point { x: i, y: 0 }, east.pos);
            east.tick();
        }
        assert_eq!(Point { x: 3, y: 0 }, east.pos);
    }
}
