use crate::{
    dot::Dot,
    grid::{Direction, Point, Receiver},
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
    position: Point,
    token: Receiver,
    edges: HashMap<Direction, Point>,
}

struct Track {
    /// Notes about nodes:
    ///
    /// Horizontal Tracks -> left: nodes.0, right: nodes.1 <br>
    /// Vertical Tracks -> top: nodes.0, bottom: nodes.1 
    nodes: (TrackNode, TrackNode),
    moving_dots: Vec<Dot>,
}

impl Cart for Track {
    type Output = Option<Dot>;
    fn tick(&mut self) -> Self::Output {
        
    }
}

pub struct ADContext {}
