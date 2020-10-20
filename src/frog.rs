use crate::ball::*;

pub enum FrogType {
    Static(Point),        // If the frog does not move
    Jumper(Vec<Point>),   // If the frog has multiple possible positions
    Slider(Point, Point), // If the frog can move along a slider
}

pub struct Frog {
    pub location: FrogType,
    pub active_ball: Ball,
    pub next_ball: Ball,
}
