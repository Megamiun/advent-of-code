use crate::y2024::d25::code_chronicle::Schema::{Key, Lock};
use itertools::Itertools;
use std::ops::Index;

#[allow(dead_code)]
pub fn get_combos(groups: &[&[String]]) -> usize {
    let (locks, keys): (Vec<Schema>, Vec<Schema>) = groups.iter().copied().map(parse)
        .partition(|schema| matches!(schema, Key( .. )));
    
    locks.iter().map(|lock| 
        keys.iter().filter(|key|(0..5).all(|pin| 5 >= lock[pin] + key[pin])).count()
    ).sum()
}

fn parse(group: &[String]) -> Schema {
    let schema = group.iter().map(|line| line.chars().collect_vec()).collect_vec();
    let pins: [usize; 5] = (0..5).map(|col|
        (0..7).filter(|row| schema[*row][col] == '#').count() - 1
    ).collect_vec().try_into().unwrap();

    if schema[0][0] == '.' {
        Key(pins)
    } else {
        Lock(pins)
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum Schema {
    Key([usize; 5]),
    Lock([usize; 5])
}

impl Index<usize> for Schema {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Key(pins) => &pins[index],
            Lock(pins) => &pins[index]
        }
    }
}
