use crate::util::bounded::Bounded;
use crate::util::coordinates::Index2D;
use crate::util::direction::Direction;
use crate::util::direction::Direction::{Left, Right};
use crate::y2024::d15::warehouse_woes::Cell::{Empty, Robot, Wall};
use itertools::Itertools;
use Cell::{Box, BoxL, BoxR};
use Direction::{Down, Up};

#[allow(dead_code)]
pub fn move_robot_single([map, directions]: &[&[String]; 2]) -> usize {
    move_robot(&mut Bounded::create_from(map, Cell::from), directions)
}

#[allow(dead_code)]
pub fn move_robot_wide([map, directions]: &[&[String]; 2]) -> usize {
    move_robot(&mut Bounded::create_from(map, Cell::from).widen(), directions)
}

fn move_robot(map: &mut Bounded<Cell>, directions: &[String]) -> usize {
    let directions = directions.iter()
        .flat_map(|line| line.chars().map(|c| Direction::from_char(c)));

    directions.fold(
        map.find_first(&Robot).unwrap(),
        |acc, dir| map.move_robot(&acc, dir));

    map.calculate()
}

impl Bounded<Cell> {
    fn widen(&self) -> Bounded<Cell> {
        let content = self.content.iter().map(|line| {
            line.iter()
                .flat_map(|cell| cell.widen())
                .collect_vec()
        }).collect_vec();

        Bounded::from(content)
    }

    fn move_robot(&mut self, robot: &Index2D, direction: Direction) -> Index2D {
        let is_horizontal = direction == Left || direction == Right;
        if self.can_move(&robot, direction, is_horizontal) {
            self.push(&robot, direction, is_horizontal);
            (robot + direction).unwrap()
        } else {
            *robot
        }
    }

    fn push(&mut self, curr: &Index2D, direction: Direction, horizontal: bool) {
        let next = (curr + direction.get_dir()).unwrap();
        let next_cell = *self.find_safe(&next);

        match next_cell {
            Box => self.push(&next, direction, horizontal),
            BoxL => {
                if !horizontal {
                    self.push(&next, direction, horizontal);
                    self.push(&(next + Right.get_dir()).unwrap(), direction, horizontal)
                } else {
                    self.push(&next, direction, horizontal)
                }
            }
            BoxR => {
                if !horizontal {
                    self.push(&next, direction, horizontal);
                    self.push(&(next + Left.get_dir()).unwrap(), direction, horizontal)
                } else {
                    self.push(&next, direction, horizontal)
                }
            }
            _ => { }
        };

        let curr_cell = *self.find_safe(&curr);
        self[&next] = curr_cell;
        self[&curr] = Empty;
    }

    fn can_move(&self, curr: &Index2D, direction: Direction, horizontal: bool) -> bool {
        let next = (curr + direction.get_dir()).unwrap();
        let next_cell = *self.find_safe(&next);

        match next_cell {
            Box => self.can_move(&next, direction, horizontal),
            BoxL => {
                if !horizontal {
                    self.can_move(&next, direction, horizontal) &&
                        self.can_move(&(next + Right.get_dir()).unwrap(), direction, horizontal)
                } else {
                    self.can_move(&next, direction, horizontal)
                }
            }
            BoxR => {
                if !horizontal {
                    self.can_move(&next, direction, horizontal) &&
                        self.can_move(&(next + Left.get_dir()).unwrap(), direction, horizontal)
                } else {
                    self.can_move(&next, direction, horizontal)
                }
            }
            Wall => false,
            _ => true
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
