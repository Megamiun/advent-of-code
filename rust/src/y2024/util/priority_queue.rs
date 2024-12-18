use std::fmt::Debug;

pub struct PriorityQueue<T: PartialOrd + Ord + Clone> {
    delegate: Vec<T>,
}

impl<T: PartialOrd + Ord + Clone + Debug> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            delegate: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.delegate.is_empty()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.delegate.pop()
    }

    pub fn push(&mut self, item: &T) {
        self.push_ordered(item, 0, self.delegate.iter().len())
    }

    fn push_ordered(&mut self, item: &T, start: usize, end: usize) {
        if start >= end {
            self.delegate.insert(start, item.clone());
            return;
        }

        let half = (end + start) / 2;

        if item >= &self.delegate[half] {
            self.push_ordered(item, start, half);
        } else {
            self.push_ordered(item, half + 1, end);
        }
    }
}
