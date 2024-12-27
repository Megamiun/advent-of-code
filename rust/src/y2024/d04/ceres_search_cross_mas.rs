use crate::util::coordinates::{Diff, Index2D};
use crate::util::bounded::Bounded;

static PERMUTATIONS: &[[(char, [Diff; 2]); 2]] = &[
    [
        ('M', [Diff(1, 1), Diff(-1, 1)]),
        ('S', [Diff(-1, -1), Diff(1, -1)]),
    ],
    [
        ('S', [Diff(1, 1), Diff(1, -1)]),
        ('M', [Diff(-1, 1), Diff(-1, -1)]),
    ],
    [
        ('S', [Diff(1, 1), Diff(-1, 1)]),
        ('M', [Diff(-1, -1), Diff(1, -1)]),
    ],
    [
        ('M', [Diff(1, 1), Diff(1, -1)]),
        ('S', [Diff(-1, 1), Diff(-1, -1)]),
    ],
];

#[allow(dead_code)]
pub fn find_all_cross_mas(lines: &[String]) -> usize {
    Bounded::from(lines).find_all_cross_mas()
}

impl Bounded<char> {
    fn find_all_cross_mas(&self) -> usize {
        (1..self.height - 1).map(|y| {
            (1..self.width - 1).filter(|&x| {
                let position = &Index2D(x, y);
                let char = self.find_safe(position);

                char == 'A' && self.is_xmas(position)
            }).count()
        }).sum()
    }

    fn is_xmas(&self, position: &Index2D) -> bool {
        PERMUTATIONS.iter().any(|permutation|
            permutation.iter().all(|(char, diff)|
                self.find_safe(&(position + diff[0]).unwrap()) == *char &&
                    self.find_safe(&(position + diff[1]).unwrap()) == *char
            ))
    }
}
