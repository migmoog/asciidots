use crate::{
    dot::Dot,
    grid::{CouldntReceive, Direction, Point, Receiver, ReceiverNode},
};
use std::collections::HashMap;

/// Carts are any object that need to experience changes on Tracks.
pub trait Cart {
    type Output;
    /// Progresses the state of the cart.
    fn tick(&mut self) -> Self::Output;
}

/// The point where a track ends
struct TrackNode {
    pos: Point,
    token: ReceiverNode,
    edges: HashMap<Direction, Point>,
}

impl TrackNode {
    fn new(pos: Point, token: ReceiverNode) -> Self {
        Self {
            pos,
            token,
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, p: Point) {
        // this point should share at least one coordinate to be a valid edge
        assert!(!(p.x != self.pos.x && p.y != self.pos.y));

        let dir = if self.pos.x < p.x {
            Direction::East
        } else if p.x < self.pos.x {
            Direction::West
        } else if self.pos.y < p.y {
            Direction::North
        } else {
            Direction::South
        };
        self.edges.insert(dir, p);
    }
}

impl Receiver for TrackNode {
    fn receive(&mut self, d: Dot) -> Result<Vec<Dot>, CouldntReceive> {
        self.token.receive(d)
    }
}

struct Track {
    nodes: (Point, Point),
    moving_dots: Vec<Dot>,
}

impl Track {
    fn dot_is_finished(&self, i: usize) -> bool {
        let d = &self.moving_dots[i];
        match d.trajectory {
            Direction::West | Direction::North => self.nodes.0 == d.pos,
            Direction::East | Direction::South => self.nodes.1 == d.pos,
        }
    }
}

impl Cart for Track {
    type Output = Vec<Dot>;
    fn tick(&mut self) -> Self::Output {
        let mut received = Vec::new();
        let mut i = 0;
        while i < self.moving_dots.len() {
            self.moving_dots[i].tick();
            if self.dot_is_finished(i) {
                let d = self.moving_dots.remove(i);
                received.push(d);
            } else {
                i += 1;
            }
        }

        received
    }
}

#[cfg(test)]
mod track_tests {
    use super::*;

    fn d(x: u32, y: u32, trajectory: Direction) -> Dot {
        Dot {
            pos: Point { x, y },
            value: 1.0,
            trajectory,
        }
    }

    #[test]
    fn single_moving_dot() {
      let mut t = Track {
        nodes: (Point::from((0, 0)), Point::from((3, 0))),
        moving_dots: vec![d(0,0, Direction::East)]
      }; 

      let mut received_dots = t.tick();
      let mut dot_ticks = 1;
      while received_dots.is_empty() {
        received_dots = t.tick();
        dot_ticks += 1;
      }

      assert!(t.moving_dots.is_empty());
      assert_eq!(dot_ticks, 3);
    }

    #[test]
    fn divergent_dots() {
        let mut t = Track {
            nodes: (Point::from((0, 1)), Point::from((0, 5))),
            moving_dots: vec![d(0, 3, Direction::North), d(0, 3, Direction::South)]
        };

        let mut received = t.tick();
        let mut tick_count = 1;
        while received.is_empty() {
            received = t.tick();
            tick_count += 1;
        }

        assert_eq!(2, tick_count);
        assert_eq!(2, received.len());
        assert_eq!(
            Point::from((0, 1)),
            received[0].pos
        );
        assert_eq!(
            Point::from((0, 5)),
            received[1].pos
        )
    }
}

pub struct ADContext {
    tracks: Vec<Track>,
    nodes: Vec<TrackNode>
}

impl ADContext {
    
}

impl Cart for ADContext {
    type Output = ();
    fn tick(&mut self) -> Self::Output {
        
    }
}