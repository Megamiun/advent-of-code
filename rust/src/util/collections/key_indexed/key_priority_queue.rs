use crate::util::collections::key_indexed::keyable::Keyable;
use std::fmt::Debug;

#[derive(Debug)]
pub struct KeyPriorityQueue<V>
where
    V: Keyable + Copy,
    V::Key: Ord + Copy + Debug,
{
    delegate: Vec<(V::Key, Vec<V>)>,
}

impl<V: Keyable> KeyPriorityQueue<V>
where
    V: Copy,
    V::Key: Ord + Copy + Debug,
{
    pub fn new() -> KeyPriorityQueue<V> {
        KeyPriorityQueue { delegate: Vec::with_capacity(64) }
    }

    pub fn is_empty(&self) -> bool {
        self.delegate.is_empty()
    }

    pub fn pop(&mut self) -> Option<V> {
        let (_, content) = self.delegate.last_mut()?;
        let item = content.pop();

        if content.is_empty() {
            self.delegate.pop();
        }

        item
    }

    pub fn push(&mut self, item: &V) {
        self.push_ordered(&item.get_key(), item, 0, self.delegate.len())
    }

    fn push_ordered(&mut self, key: &V::Key, item: &V, start: usize, end: usize) {
        if start >= end {
            let mut values = Vec::with_capacity(16);
            values.push(*item);
            self.delegate.insert(start, (*key, values));
            return;
        }

        let half = (end + start) / 2;

        if let Some((index_key, index_list)) = self.delegate.get_mut(half) {
            if key == index_key {
                index_list.push(*item)
            } else if key >= index_key {
                self.push_ordered(key, item, start, half);
            } else {
                self.push_ordered(key, item, half + 1, end);
            }
        }
    }

    pub const fn iter(&'_ mut self) -> Iter<'_, V> {
        Iter(self)
    }
}

pub struct Iter<'a, V>(&'a mut KeyPriorityQueue<V>)
where
    V: Keyable + Copy,
    V::Key: Ord + Copy + Debug;

impl<'a, V> Iterator for Iter<'a, V>
where
    V: Keyable + Copy,
    V::Key: Ord + Copy + Debug {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
