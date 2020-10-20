#![allow(dead_code)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod ball;
mod frog;

pub use ball::*;
pub use frog::Frog;
