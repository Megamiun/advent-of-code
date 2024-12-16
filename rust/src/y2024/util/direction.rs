use crate::util::Diff;
use crate::y2024::util::direction::Direction::{Down, Left, Right, Up};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub const VALUES: [Direction; 4] = [Up, Right, Down, Left];
    
    pub fn get_dir(&self) -> Diff {
        match self {
            Up => Diff(0, -1),
            Right =>  Diff(1, 0),
            Down => Diff(0, 1),
            Left =>  Diff(-1, 0)
        }
    }

    pub fn get_reverse(&self) -> Direction {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right
        }
    }

    pub fn get_clockwise(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
    
    pub fn get_counter_clockwise(&self) -> Direction {
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down
        }
    }
}
