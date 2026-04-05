#![allow(dead_code)]

pub struct FixedSizeMinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> FixedSizeMinHeap<T> {
    pub fn new(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size),
        }
    }

    pub fn from(data: impl IntoIterator<Item = T>) -> Self {
        Self {
            data: data.into_iter().collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.data.capacity()
    }

    pub fn insert(&mut self, _item: T) {
        todo!()
    }

    pub fn pop(&mut self) -> Option<&T> {
        todo!()
        // if self.data.len()
        // let it = self.data.swap_remove(0);
        // self.hea
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn contains(&self, item: &T) -> bool {
        self.data.contains(item)
    }

    fn parent_index(idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child_index(idx: usize) -> usize {
        2 * idx + 1
    }

    fn right_child_index(idx: usize) -> usize {
        2 * idx + 2
    }

    fn parent(&self, idx: usize) -> Option<&T> {
        self.data.get(Self::parent_index(idx))
    }

    fn left_child(&self, idx: usize) -> Option<&T> {
        self.data.get(Self::left_child_index(idx))
    }

    fn right_child(&self, idx: usize) -> Option<&T> {
        self.data.get(Self::right_child_index(idx))
    }

    fn swap(&mut self, idx1: usize, idx2: usize) {
        self.data.swap(idx1, idx2)
    }

    fn heapify_up(&mut self) {
        todo!()
    }

    fn heapify_down(&mut self) {
        todo!()
    }
}
