#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BallColor {
    Blue,
    Yellow,
    Red,
    Green,
    Purple,
    White,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BallEffect {
    None,
    Slow,
    Reverse,
    Bomb,
    Visor,
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Ball {
    pub coordinates: Point,
    pub is_reachable: bool, // false if it is in a tunnel for example
    pub color: BallColor,
    pub effect: BallEffect,
}

#[derive(Debug)]
pub struct BallSequence {
    pub balls: Vec<Ball>,
}

#[derive(Debug)]
pub struct GameState {
    pub groups: Vec<BallSequence>,
}

impl BallSequence {
    fn clear_at(&mut self, index: usize) -> Option<()> {
        let color_to_clear = self.balls[index].color;

        let balls_to_clear: Vec<usize> = self.balls[..index]
            .iter()
            .rev()
            .take_while(|b| b.color == color_to_clear)
            .enumerate()
            .map(|(i, _)| index - i)
            .chain(
                self.balls[index..]
                    .iter()
                    .take_while(|b| b.color == color_to_clear)
                    .enumerate()
                    .map(|(i, _)| index + i),
            )
            .collect();

        // If more than 3 balls are contiguous, actually clear them
        if balls_to_clear.len() >= 3 {
            self.balls
                .drain((balls_to_clear.iter().min()?)..(balls_to_clear.iter().max()?));
        }

        Some(())
    }
}
