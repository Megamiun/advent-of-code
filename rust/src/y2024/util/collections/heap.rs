use crate::y2024::util::collections::key_indexed::keyable::Keyable;

pub struct Heap<V: Keyable + Copy> {
    delegate: Vec<V>
}

impl<V> Heap<V> where V: Keyable + Copy {
    pub fn new() -> Heap<V> {
        Heap { delegate: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.delegate.is_empty()
    }

    pub fn pop(&mut self) -> Option<V> {
        if self.delegate.len() == 0 {
            return None
        }
        
        let first = self.delegate.swap_remove(0);
        self.sift_down(0);
        Some(first)
    }

    pub fn push(&mut self, item: &V) {
        self.delegate.push(*item);
        self.sift_up(self.delegate.len() - 1);
    }

    fn sift_up(&mut self, node: usize) {
        if node == 0 {
            return;
        }

        let parent = node / 2;
        if self.delegate[node].get_key() < self.delegate[parent].get_key() {
            self.delegate.swap(node, parent);
            self.sift_up(parent)
        }
    }

    fn sift_down(&mut self, node: usize) {
        let left = ((node + 1) * 2) + 1;
        let right = left + 1;

        if right >= self.delegate.len() || self.delegate[left].get_key() < self.delegate[right].get_key() {
            self.swap_down(node, left);
        } else {
            self.swap_down(node, right);
        }
    }

    fn swap_down(&mut self, parent: usize, node: usize) {
        if node < self.delegate.len() && self.delegate[node].get_key() < self.delegate[parent].get_key() {
            self.delegate.swap(parent, node);
            self.sift_down(node)
        }
    }
}
