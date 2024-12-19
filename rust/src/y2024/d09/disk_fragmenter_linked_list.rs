use derive_more::Display;
use std::fmt::Formatter;
use std::ptr::null_mut;

pub fn reorder_linked_list(lines: &[String]) -> usize {
    let chuncked = get_file_space_pair(lines);

    let files = chuncked
        .iter()
        .enumerate()
        .scan(0usize, |prev_end, (index, (file_size, blank_size))| {
            let start = *prev_end;
            // This scan MAKES you mutate the state, instead of reusing the previous one 
            *prev_end = start + file_size + blank_size.unwrap_or(0);
            Some((start, *file_size, *prev_end, index))
        })
        .map(|(start, size, _, id)| (start, Content { file: id, size }))
        .collect::<Vec<_>>();

    let mut list = SortedLinkedList::new();

    files.iter().for_each(|(start, file)| {
        list.insert_at(*start, *file);
    });

    files.iter().rev().for_each(|(_, file)| {
        list.relocate(*file);
    });

    list.get_iter().map(|node| node.get_value()).sum()
}

struct SortedLinkedList<'a> {
    head: *mut Node<'a>,
}

impl<'a> SortedLinkedList<'a> {
    fn new() -> SortedLinkedList<'a> {
        SortedLinkedList { head: null_mut() }
    }

    fn relocate(&mut self, file: Content) {
        if !self.head.is_null() {
            unsafe { (*self.head).relocate(file) }
        }
    }

    fn insert_at(&mut self, start: usize, file: Content) {
        if self.head.is_null() || start < unsafe { &mut *self.head }.start {
            self.head = Node::get_raw_ptr(start, file, self.head);
        } else {
            unsafe { &mut *self.head }.insert_at(start, file);
        }
    }

    fn get_iter(&self) -> NodeIterator<'a> {
        if self.head.is_null() {
            return NodeIterator(None);
        }

        NodeIterator(Some(unsafe { *self.head }))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Display, Debug)]
#[display("[{file}; {size}]")]
struct Content {
    file: usize,
    size: usize,
}

// TODO Check memory leaks and Implement Drop to clean up
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Node<'a> {
    start: usize,
    file: Content,
    next: *mut Node<'a>,
}

impl<'a> Node<'a> {
    fn get_raw_ptr(start: usize, file: Content, self_next: *mut Node<'a>) -> *mut Node<'a> {
        Box::into_raw(Box::new(Node {
            start,
            file,
            next: self_next,
        }))
    }

    fn remove(&mut self, file: Content) {
        if self.next.is_null() {
            return;
        }

        let next = unsafe { &mut *self.next };
        if next.file == file {
            self.next = next.next;
        } else {
            next.remove(file);
        }
    }

    fn relocate(&mut self, file: Content) {
        if self.file == file {
            return;
        }

        if self.next.is_null() {
            self.next = Node::get_raw_ptr(self.get_end(), file, null_mut());
            return;
        }

        let next = unsafe { &mut *self.next };
        if self.get_end() + file.size <= next.start {
            self.next = Node::get_raw_ptr(self.get_end(), file, next);
            next.remove(file)
        } else {
            next.relocate(file);
        }
    }

    fn insert_at(&mut self, start: usize, file: Content) {
        if self.start == start {
            self.file = file;
            return;
        }

        if self.next.is_null() || start < unsafe { &mut *self.next }.start {
            self.next = Node::get_raw_ptr(start, file, self.next)
        } else {
            unsafe { &mut *self.next }.insert_at(start, file);
        }
    }

    fn get_end(&self) -> usize {
        self.start + self.file.size
    }

    fn get_value(&self) -> usize {
        (self.start..self.get_end()).sum::<usize>() * self.file.file
    }
}

struct NodeIterator<'a>(Option<Node<'a>>);

impl<'a> Iterator for NodeIterator<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.0?;
        self.0 = Some(node.next)
            .take_if(|n| !n.is_null())
            .map(|t| unsafe { *t });
        
        Some(node)
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.next.is_null() {
            write!(f, "[{}, {:?}] -> {}", self.start, self.file, unsafe { *self.next })
        } else {
            write!(f, "[{}, {:?}]", self.start, self.file)
        }
    }
}

fn get_file_space_pair(lines: &[String]) -> Vec<(usize, Option<usize>)> {
    lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|s| (s[0], s.get(1).copied()))
        .collect::<Vec<_>>()
}

