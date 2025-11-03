#![allow(dead_code)]

pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn from(data: impl IntoIterator<Item = T>) -> Self {
        let mut data: Vec<_> = data.into_iter().collect();
        data.sort();
        Self { data }
    }

    pub fn insert(&mut self, item: T) {
        self.data.push(item);
        self.heapify_up();
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.len() < 2 {
            self.data.pop()
        } else {
            let it = self.data.swap_remove(0);
            self.heapify_down();
            Some(it)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.data.contains(value)
    }

    fn parent_index(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child_index(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn right_child_index(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    fn parent(&self, idx: usize) -> Option<&T> {
        self.data.get(self.parent_index(idx))
    }

    fn left_child(&self, idx: usize) -> Option<&T> {
        self.data.get(self.left_child_index(idx))
    }

    fn right_child(&self, idx: usize) -> Option<&T> {
        self.data.get(self.right_child_index(idx))
    }

    fn swap(&mut self, idx1: usize, idx2: usize) {
        self.data.swap(idx1, idx2)
    }

    fn heapify_up(&mut self) {
        let mut i = self.data.len() - 1;
        while i > 0 {
            let curr = self.data.get(i).unwrap();
            let parent = self.parent(i).unwrap();
            if curr > parent {
                break;
            }

            let parent_index = self.parent_index(i);
            self.swap(i, parent_index);
            i = parent_index;
        }
    }

    fn heapify_down(&mut self) {
        let mut i: usize = 0;
        loop {
            let len = self.data.len();
            let left_idx = self.left_child_index(i);
            let right_idx = self.right_child_index(i);
            let mut min_index: usize = i;

            if left_idx < len && self.data[left_idx] < self.data[min_index] {
                min_index = left_idx;
            }
            if right_idx < len && self.data[right_idx] < self.data[min_index] {
                min_index = right_idx;
            }

            if min_index == i {
                break;
            }

            self.swap(i, min_index);
            i = min_index;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn empty() {
        let heap = MinHeap::<i32>::new();
        assert_eq!(heap.peek(), None);
    }

    #[test]
    fn len() {
        let mut heap = MinHeap::new();
        heap.insert(0);
        heap.insert(1);
        heap.insert(2);
        heap.insert(3);
        assert_eq!(heap.len(), 4);
    }

    #[test]
    fn with_capacity() {
        let mut heap = MinHeap::with_capacity(4);
        assert_eq!(heap.capacity(), 4);
        heap.insert(0);
        assert_eq!(heap.capacity(), 4);
        heap.insert(0);
        heap.insert(0);
        heap.insert(0);
        assert_eq!(heap.capacity(), 4);
        heap.insert(0);
        assert_ne!(heap.capacity(), 4);
    }

    #[test]
    fn smallest_item() {
        let mut heap = MinHeap::new();
        heap.insert(4);
        heap.insert(5);
        heap.insert(2);
        heap.insert(9);
        assert_eq!(heap.peek(), Some(&2));
        heap.insert(1);
        assert_eq!(heap.peek(), Some(&1))
    }

    #[test]
    fn pop() {
        let mut heap = MinHeap::new();
        heap.insert(4);
        heap.insert(5);
        heap.insert(2);
        heap.insert(9);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(9));
    }
}
