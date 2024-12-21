use crate::util::Index2D;
use crate::y2024::d15::warehouse_woes::Cell::{Empty, Robot, Wall};
use crate::y2024::util::bounded::Bounded;
use crate::y2024::util::direction::Direction;
use crate::y2024::util::direction::Direction::{Left, Right};
use derive_more::Display;
use std::fmt::Formatter;
use Cell::{Box, BoxL, BoxR};
use Direction::{Down, Up};

pub fn move_robot_single(groups: &[&[String]]) -> usize {
    let mut map = Bounded::create_from(groups[0], Cell::from);

    let directions = groups[1]
        .iter()
        .flat_map(|line| line.chars().map(|c| Direction::from_char(c)))
        .collect::<Vec<_>>();

    directions.iter().for_each(|dir| { map.move_robot(*dir); });
    map.calculate()
}

pub fn move_robot_wide(groups: &[&[String]]) -> usize {
    let mut map = Bounded::create_from(groups[0], Cell::from).widen();

    let directions = groups[1]
        .iter()
        .flat_map(|line| line.chars().map(|c| Direction::from_char(c)))
        .collect::<Vec<_>>();

    directions.iter().for_each(|dir| { map.move_robot(*dir); });
    map.calculate()
}

impl Bounded<Cell> {
    fn widen(&self) -> Bounded<Cell> {
        let content = self.content
            .iter()
            .map(|line| {
                line.iter()
                    .flat_map(|cell| cell.widen())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Bounded::from(&content)
    }

    fn move_robot(&mut self, direction: Direction) -> Option<bool> {
        let robot = self.find_first(&Robot)?;

        let is_horizontal = direction == Left || direction == Right;
        if self.can_move(&robot, direction, is_horizontal)? {
            self.push(&robot, direction, is_horizontal);
        }
        Some(true)
    }

    fn push(
        &mut self,
        curr: &Index2D,
        direction: Direction,
        horizontal: bool,
    ) -> Option<bool> {
        let next = (curr + direction.get_dir())?;
        let next_cell = *self.find(&next)?;

        match next_cell {
            Box => self.push(&next, direction, horizontal),
            BoxL => {
                if !horizontal {
                    self.push(&next, direction, horizontal)?;
                    self.push(&(next + Right.get_dir())?, direction, horizontal)
                } else {
                    self.push(&next, direction, horizontal)
                }
            }
            BoxR => {
                if !horizontal {
                    self.push(&next, direction, horizontal)?;
                    self.push(&(next + Left.get_dir())?, direction, horizontal)
                } else {
                    self.push(&next, direction, horizontal)
                }
            }
            _ => Some(true),
        };

        let curr_cell = *self.find(&curr)?;
        self.set(&next, curr_cell);
        self.set(&curr, Empty);
        Some(true)
    }

    fn can_move(
        &self,
        curr: &Index2D,
        direction: Direction,
        horizontal: bool,
    ) -> Option<bool> {
        let next = (curr + direction.get_dir())?;
        let next_cell = *self.find(&next)?;

        match next_cell {
            Box => self.can_move(&next, direction, horizontal),
            BoxL => {
                if !horizontal {
                    self.can_move(&next, direction, horizontal)?;
                    self.can_move(&(next + Right.get_dir())?, direction, horizontal)
                } else {
                    self.can_move(&next, direction, horizontal)
                }
            }
            BoxR => {
                if !horizontal {
                    self.can_move(&next, direction, horizontal)?;
                    self.can_move(&(next + Left.get_dir())?, direction, horizontal)
                } else {
                    self.can_move(&next, direction, horizontal)
                }
            }
            Wall => None,
            _ => Some(true),
        }
    }

    fn calculate(&self) -> usize {
        self.get_all_coordinates_with_content_iter()
            .filter(|(_, &cell)| cell == Box || cell == BoxL)
            .map(|(pos, _)| (pos.1 * 100) + pos.0)
            .sum()
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Robot,
    Empty,
    Box,
    Wall,
    BoxL,
    BoxR,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Robot => '@',
            Empty => ' ',
            Box => 'O',
            Wall => '#',
            BoxL => '[',
            BoxR => ']',
        };

        write!(f, "{char}")
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Wall,
            'O' => Box,
            '@' => Robot,
            _ => Empty,
        }
    }
}

impl Cell {
    fn widen(&self) -> [Cell; 2] {
        match self {
            Robot => [Robot, Empty],
            Empty => [Empty, Empty],
            Wall => [Wall, Wall],
            Box | BoxL | BoxR => [BoxL, BoxR],
        }
    }
}

impl Direction {
    fn from_char(value: char) -> Self {
        match value {
            '^' => Up,
            '>' => Right,
            '<' => Left,
            _ => Down,
        }
    }
}
