use crate::util::coordinates::Diff;
use crate::util::direction::Direction::{Down, Left, Right, Up};
use derive_more::From;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl From<char> for Direction {
    fn from(char: char) -> Direction {
        match char {
            '^' => Up,
            '>' => Right,
            'v' => Down,
            _ => Left
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Up => '^',
            Right => '>',
            Down => 'v',
            Left => '<'
        }
    }
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