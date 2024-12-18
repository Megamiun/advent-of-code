pub struct KeyPriorityQueue<K: PartialOrd + Ord + Clone, I: Clone> {
    delegate: Vec<(K, Vec<I>)>,
    get_key: fn(&I) -> K
}

impl<K: PartialOrd + Ord + Clone, I: Clone> KeyPriorityQueue<K, I> {
    pub fn is_empty(&self) -> bool {
        self.delegate.is_empty()
    }

    pub fn pop(&mut self) -> Option<I> {
        let (_, content) = self.delegate.last_mut()?;
        let item = content.pop();

        if content.is_empty() {
            self.delegate.pop();
        }

        item
    }
    
    pub fn stack_queue(get_key: fn(&I) -> K) -> KeyPriorityQueue<K, I> {
        KeyPriorityQueue {
            delegate: Vec::new(),
            get_key
        }
    }

    pub fn push(&mut self, item: &I) {
        self.push_ordered(item, &(self.get_key)(item), 0, self.delegate.iter().len())
    }

    fn push_ordered(&mut self, item: &I, key: &K, start: usize, end: usize) {
        if start >= end {
            self.delegate.insert(start, (key.clone(), vec![item.clone()]));
            return;
        }

        let half = (end + start) / 2;

        if let Some((index_key, index_list)) = &mut self.delegate.get_mut(half) {
            if key == index_key {
                index_list.push(item.clone())
            } else if key >= index_key {
                self.push_ordered(item, key, start, half);
            } else {
                self.push_ordered(item, key, half + 1, end);
            }
        }
    }
}
