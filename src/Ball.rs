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
    x: f32,
    y: f32,
}

pub struct Ball {
    coordinates : Point,
    is_reachable : boolean, // false if it is in a tunnel for example
    color : BallColor,
    effect : BallEffect,
}