use crate::util::collections::key_indexed::keyable::Keyable;

pub struct KeyHeap<V: Keyable + Copy> where V::Key: Eq + Copy {
    delegate: Vec<(V::Key, Vec<V>)>,
}

impl<V> KeyHeap<V> where
    V: Keyable + Copy,
    V::Key: Eq + Copy,
{
    pub fn new() -> KeyHeap<V> {
        KeyHeap {
            delegate: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.delegate.is_empty()
    }

    pub fn pop(&mut self) -> Option<V> {
        let (_, content) = self.delegate.first_mut()?;
        let item = content.pop();

        if content.is_empty() {
            self.delegate.swap_remove(0);
            self.sift_down(0);
        }

        item
    }

    pub fn push(&mut self, item: &V) {
        let item_key = item.get_key();
        if let Some(node) = self.find_existent(&item_key, 0) {
            self.delegate[node].1.push(*item);
            return
        };

        self.delegate.push((item_key, vec![*item]));
        self.sift_up(self.delegate.len() - 1);
    }

    fn find_existent(&mut self, key: &V::Key, node: usize) -> Option<usize> {
        if node >= self.delegate.len() {
            return None
        }

        let node_key = self.delegate[node].0;
        if *key < node_key {
            return None
        }
        
        if *key == node_key {
            return Some(node)
        }
        
        let left = ((node + 1) * 2) + 1;
        let right = left + 1;
        
        self.find_existent(key, left).or(self.find_existent(key, right))
    }

    fn sift_up(&mut self, node: usize) {
        if node == 0 {
            return;
        }

        let parent = node / 2;
        if self.delegate[node].0 < self.delegate[parent].0 {
            self.delegate.swap(node, parent);
            self.sift_up(parent)
        }
    }

    fn sift_down(&mut self, node: usize) {
        let left = ((node + 1) * 2) + 1;
        let right = left + 1;

        if right >= self.delegate.len() || self.delegate[left].0 < self.delegate[right].0 {
            self.swap_down(node, left);
        } else {
            self.swap_down(node, right);
        }
    }

    fn swap_down(&mut self, parent: usize, node: usize) {
        if node < self.delegate.len() && self.delegate[node].0 < self.delegate[parent].0 {
            self.delegate.swap(parent, node);
            self.sift_down(node)
        }
    }
}
