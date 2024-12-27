use crate::y2024::d09::helper::get_files_and_blanks;
use derive_more::Display;
use itertools::Itertools;
use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;

#[allow(dead_code)]
pub fn reorder_linked_list(lines: &[String]) -> usize {
    let mut list = SortedLinkedList::new();

    let (files, _) = get_files_and_blanks(lines);
    let files = files.iter()
        .map(|[start, size, id]| (start, Content { file: *id, size: *size }))
        .inspect(|(&start, file)| list.insert_at(start, *file))
        .collect_vec();

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
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Node<'a> {
    start: usize,
    file: Content,
    next: *mut Node<'a>,
}

impl<'a> Node<'a> {
    fn get_raw_ptr(start: usize, file: Content, self_next: *mut Node<'a>) -> *mut Node<'a> {
        Box::into_raw(Box::new(Node { start, file, next: self_next }))
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
            unsafe { &mut *self.next }.remove(file)
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

impl<'a> Display for SortedLinkedList<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.head.is_null() {
            write!(f, "{}", unsafe { *self.head })
        } else {
            write!(f, "[]")
        }
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
