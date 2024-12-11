use crate::util::{Diff, Index2D};
use crate::y2024::util::bounded::Bounded;

static XMAS: &[char] = &['X', 'M', 'A', 'S'];
static SAMX: &[char] = &['S', 'A', 'M', 'X'];

pub fn find_all_xmas_collecting(lines: &[String]) -> usize {
    Bounded::from(lines).find_all_xmas_collecting()
}

impl Bounded<char> {
    fn find_all_xmas_collecting(&self) -> usize {
        let lines = [
            &self.content,
            &self.collect_columns(),
            &self.collect_diagonals()
        ].iter()
            .flat_map(|s| *s)
            .cloned()
            .collect::<Vec<_>>();

        lines.iter().map(|line|
            line
                .windows(4)
                .filter(|&window| window == XMAS || window == SAMX)
                .count()
        ).sum()
    }

    fn collect_columns(&self) -> Vec<Vec<char>> {
        let diff = &Diff(0, 1);

        (0..self.width)
            .map(|x| self.collect_until_end(&Index2D(x, 0), diff))
            .collect::<Vec<_>>()
    }

    fn collect_diagonals(&self) -> Vec<Vec<char>> {
        let diff_right = &Diff(1, 1);
        let diff_left = &Diff(-1, 1);

        let from_top_to_right = (0..self.width)
            .map(|x| self.collect_until_end(&Index2D(x, 0), diff_right))
            .collect::<Vec<_>>();

        let from_top_to_left = (0..self.width)
            .map(|x| self.collect_until_end(&Index2D(x, 0), diff_left))
            .collect::<Vec<_>>();

        let from_left = (1..self.height)
            .map(|y| self.collect_until_end(&Index2D(0, y), diff_right))
            .collect::<Vec<_>>();

        let from_right = (1..self.height)
            .map(|y| self.collect_until_end(&Index2D(self.width - 1, y), diff_left))
            .collect::<Vec<_>>();

        [from_top_to_right, from_left, from_top_to_left, from_right].concat().to_vec()
    }

    fn collect_until_end(&self, start: &Index2D, diff: &Diff) -> Vec<char> {
        std::iter::successors(
            Option::from(*start),
            |prev| prev.add(*diff),
        ).map_while(|pos| self.find(pos))
            .copied().collect()
    }
}
