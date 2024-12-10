use derive_more::Display;
use std::collections::VecDeque;
use std::fmt::Formatter;
use std::iter::repeat_with;
use std::ptr::null_mut;

pub fn reorder(lines: &[String]) -> usize {
    let chuncked = &lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|s| (s[0], s.get(1).cloned()))
        .collect::<Vec<_>>();

    let files = chuncked
        .iter()
        .enumerate()
        .scan(0usize, |prev_end, (index, (file_size, blank_size))| {
            let start = *prev_end;
            *prev_end = start + file_size + blank_size.unwrap_or(0);
            Some((start, *file_size, *prev_end, index))
        })
        .map(|(start, size, _, id)| (start, Content { file: id, size }))
        .collect::<Vec<_>>();

    // Moving to SortedLinkedList somehow breaks this
    let mut node = Node {
        start: 0,
        file: files[0].1,
        next: None,
    };

    files.iter().skip(1).for_each(|(start, file)| {
        node.insert_at(*start, *file);
    });

    files.iter().rev().for_each(|(_, file)| {
        node.relocate(*file);
    });

    node.get_iter().map(|node| node.get_value()).sum()
}

struct SortedLinkedList {
    head: *mut Node,
}

impl SortedLinkedList {
    fn new() -> SortedLinkedList {
        SortedLinkedList { head: null_mut() }
    }

    fn relocate(&mut self, file: Content) {
        if !self.head.is_null() {
            unsafe { (*self.head).relocate(file) }
        }
    }

    fn insert_at(&mut self, start: usize, file: Content) {
        if self.head.is_null() {
            self.head = Node::as_raw_ptr(start, file, null_mut());
        } else {
            let head = &mut unsafe { *self.head };

            println!("Before head: {}", head);
            if start < head.start {
                self.head = Node::as_raw_ptr(start, file, self.head);
            } else {
                head.insert_at(start, file);
            }
            println!("After head: {}", head)
        };
    }

    fn get_iter(&self) -> NodeIterator {
        if self.head.is_null() {
            return NodeIterator(None);
        }

        NodeIterator(Some(unsafe { &*self.head }))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug)]
#[display("[{file}; {size}]")]
struct Content {
    file: usize,
    size: usize,
}

// TODO Check memory leaks and Implement Drop to clean up
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node {
    start: usize,
    file: Content,
    next: Option<*mut Node>,
}

impl Node {
    fn as_raw_ptr(start: usize, file: Content, self_next: *mut Node) -> *mut Node {
        Box::into_raw(Box::new(Node {
            start,
            file,
            next: Some(self_next).take_if(|s| !s.is_null()),
        }))
    }

    fn remove(&mut self, file: Content) {
        if let Some(mut next_ptr) = &self.next {
            let next = unsafe { &mut *next_ptr };
            if next.file == file {
                self.next = next.next;
            } else {
                next.remove(file);
            }
        }
    }

    fn relocate(&mut self, file: Content) {
        if self.file == file {
            return;
        }

        if let Some(mut next_ptr) = &self.next {
            let next = unsafe { &mut *next_ptr };
            if self.get_end() + file.size <= next.start {
                self.next = Some(Node::as_raw_ptr(self.get_end(), file, next));
                next.remove(file)
            } else {
                next.relocate(file);
            }
        } else {
            self.next = Some(Node::as_raw_ptr(self.get_end(), file, null_mut()));
        }
    }

    fn insert_at(&mut self, start: usize, file: Content) {
        if self.start == start {
            self.file = file;
            return;
        }

        if let Some(mut next_ptr) = &self.next {
            let next = unsafe { &mut *next_ptr };
            if next.start > start {
                self.next = Some(Node::as_raw_ptr(start, file, next))
            } else {
                next.insert_at(start, file);
            }
        } else {
            self.next = Some(Node::as_raw_ptr(start, file, null_mut()))
        }
    }

    fn get_end(&self) -> usize {
        self.start + self.file.size
    }

    fn get_value(&self) -> usize {
        (self.start..self.get_end()).sum::<usize>() * self.file.file
    }

    fn get_iter(&self) -> NodeIterator {
        NodeIterator(Some(self))
    }
}

struct NodeIterator<'a>(Option<&'a Node>);

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = unsafe { node.next.map(|t| &*t) };
            node
        })
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.next.is_none() {
            write!(f, "[{}, {:?}] -> {}", self.start, self.file, unsafe { *self.next.unwrap() })
        } else {
            write!(f, "[{}, {:?}]", self.start, self.file)
        }
    }
}

fn get_from_back(numbers: &mut VecDeque<usize>, total: &mut Vec<usize>, amount: usize) {
    repeat_with(|| numbers.pop_back())
        .take(amount)
        .flatten()
        .for_each(|s| total.push(s));
}

fn get_from_front(numbers: &mut VecDeque<usize>, total: &mut Vec<usize>, amount: usize) {
    repeat_with(|| numbers.pop_front())
        .take(amount)
        .flatten()
        .for_each(|s| total.push(s));
}
