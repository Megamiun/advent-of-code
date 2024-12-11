use crate::util::{Diff, Index2D};
use crate::y2024::util::bounded::Bounded;

static DIRECTIONS: &[Diff] = &[Diff(1, 1), Diff(1, 0), Diff(1, -1), Diff(0, 1), Diff(0, -1), Diff(-1, 1), Diff(-1, 0), Diff(-1, -1)];

static MAS: &[char] = &['M', 'A', 'S'];

pub fn find_all_xmas_simple(lines: &[String]) -> usize {
    Bounded::from(lines).find_all_xmas_simple()
}

impl Bounded<char> {
    fn find_all_xmas_simple(&self) -> usize {
        (0..self.height).map(|y| {
            (0..self.width).map(|x| {
                let position = Index2D(x, y);
                let char = self.find_safe(position);

                match char {
                    'X' => DIRECTIONS.iter().filter(|&dir| self.is_valid(&position, dir, MAS)).count(),
                    _ => 0
                }
            }).sum::<usize>()
        }).sum()
    }

    fn is_valid(&self, start: &Index2D, diff: &Diff, word: &[char]) -> bool {
        let value = std::iter::successors(
            start.add(*diff),
            |prev| prev.add(*diff)
        )
            .map_while(|pos| self.find(pos))
            .take(word.len())
            .copied()
            .collect::<Vec<_>>();
        
        value == *word
    }
}
