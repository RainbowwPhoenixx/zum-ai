use crate::ball::*;

pub struct Frog {
    pub coordinates : Point,
    pub active_ball : Ball,
    pub next_ball : Ball,
}