use crate::util::bounded::Bounded;
use crate::util::coordinates::{Diff, Index2D};
use itertools::chain;

static XMAS: &[char] = &['X', 'M', 'A', 'S'];
static SAMX: &[char] = &['S', 'A', 'M', 'X'];

#[allow(dead_code)]
pub fn find_all_xmas_collecting(lines: &[String]) -> usize {
    Bounded::from(lines).find_all_xmas_collecting()
}

impl Bounded<char> {
    fn find_all_xmas_collecting(&self) -> usize {
        chain!(
            self.content.clone(),
            self.collect_columns(),
            self.collect_diagonals()
        ).map(|line|
            line.windows(4)
                .filter(|&window| window == XMAS || window == SAMX)
                .count()
        ).sum()
    }

    fn collect_columns(&self) -> Vec<Vec<char>> {
        let diff = &Diff(0, 1);

        (0..self.width)
            .map(|x| self.collect_until_end(&Index2D(x, 0), diff))
            .collect()
    }

    fn collect_diagonals(&self) -> Vec<Vec<char>> {
        let diff_right = &Diff(1, 1);
        let diff_left = &Diff(-1, 1);

        let from_top_to_right = (0..self.width)
            .map(|x| self.collect_until_end(&Index2D(x, 0), diff_right));

        let from_top_to_left = (0..self.width)
            .map(|x| self.collect_until_end(&Index2D(x, 0), diff_left));

        let from_left = (1..self.height)
            .map(|y| self.collect_until_end(&Index2D(0, y), diff_right));

        let from_right = (1..self.height)
            .map(|y| self.collect_until_end(&Index2D(self.width - 1, y), diff_left));
        
        chain!(from_top_to_right, from_top_to_left, from_left, from_right).collect()
    }

    fn collect_until_end(&self, start: &Index2D, diff: &Diff) -> Vec<char> {
        std::iter::successors(Option::from(*start), |prev| prev + diff)
            .map_while(|pos| self.find(&pos))
            .copied().collect()
    }
}
