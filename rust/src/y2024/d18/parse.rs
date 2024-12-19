use crate::util::Index2D;
use crate::y2024::util::bounded::Bounded;

pub fn parse(line: &String) -> Index2D {
    if let [x, y] = line.split(",").collect::<Vec<_>>().as_slice() {
        Index2D(
            usize::from_str_radix(*x, 10).unwrap(),
            usize::from_str_radix(*y, 10).unwrap(),
        )
    } else {
        panic!("{line} can not be parsed to coordinate");
    }
}


impl Bounded<bool> {
    pub fn create_using(dimensions: usize, bytes: &[Index2D]) -> Bounded<bool> {
        let mut content = vec![vec![false; dimensions]; dimensions];

        bytes
            .iter()
            .for_each(|Index2D(x, y)| content[*y][*x] = true);

        Bounded {
            content,
            height: dimensions,
            width: dimensions,
        }
    }
}