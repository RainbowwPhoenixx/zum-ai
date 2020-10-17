#[derive(Copy, Clone, PartialEq)]
pub enum BallColor {
    Blue,
    Yellow,
    Red,
    Green,
    Purple,
    White,
}

pub enum BallEffect {
    None,
    Slow,
    Reverse,
    Bomb,
    Visor,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Ball {
    pub coordinates : Point,
    pub is_reachable : bool, // false if it is in a tunnel for example
    pub color : BallColor,
    pub effect : BallEffect,
}