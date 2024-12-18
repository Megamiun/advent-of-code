use crate::util::Index2D;
use crate::y2024::util::bounded::Bounded;
use crate::y2024::util::priority_queue::PriorityQueue;
use rustc_hash::FxHashMap;
use std::iter::successors;

pub fn find_min_steps_after(lines: &[String], dimensions: usize, bytes: usize) -> usize {
    let to_fall = lines.iter().map(parse).take(bytes).collect::<Vec<_>>();

    Bounded::create_using(dimensions, &to_fall)
        .get_min_path_for_exit()
        .unwrap().len()
}

pub fn find_min_blocker(lines: &[String], dimensions: usize) -> String {
    let to_fall = lines.iter().map(parse).collect::<Vec<_>>();

    let mut prev_path = vec![to_fall[0]];

    for curr in 0..to_fall.len() {
        let falling_stone = to_fall[curr];
        if !prev_path.contains(&falling_stone) {
            continue;
        }

        let map = Bounded::create_using(dimensions, &to_fall[..curr + 1]);
        if let Some(path) = map.get_min_path_for_exit() {
            prev_path = path
        } else { 
            return format!("{},{}", falling_stone.0, falling_stone.1)
        }
    }
    
    panic!("Path is not blocked")
}

impl Bounded<bool> {
    fn create_using(dimensions: usize, bytes: &[Index2D]) -> Bounded<bool> {
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

    fn get_min_path_for_exit(&self) -> Option<Vec<Index2D>> {
        let mut visited = FxHashMap::<Index2D, Option<Index2D>>::with_capacity_and_hasher(self.width * self.width, Default::default());
        let mut to_visit = PriorityQueue::<(usize, Index2D, Option<Index2D>)>::heap_queue();
        to_visit.push_heap(&(0, Index2D(0, 0), None));

        let end = Index2D(self.width - 1, self.height - 1);
        
        while !to_visit.is_empty() {
            let visit = to_visit.pop().unwrap();
            let (score, curr, previous) = visit.as_ref();

            if self.find_safe(curr) || visited.contains_key(curr) {
                continue;
            }

            visited.insert(*curr, *previous);

            if *curr == end {
                return Some(successors(Some(end), |prev| visited[prev]).collect());
            }

            for adj in self.find_adjacent(&curr) { 
                to_visit.push_heap(&(score + 1, adj, Some(*curr)));
            }
        }

        None
    }
}

fn parse(line: &String) -> Index2D {
    if let [x, y] = line.split(",").collect::<Vec<_>>().as_slice() {
        Index2D(
            usize::from_str_radix(*x, 10).unwrap(),
            usize::from_str_radix(*y, 10).unwrap(),
        )
    } else {
        panic!("{line} can not be parsed to coordinate");
    }
}
