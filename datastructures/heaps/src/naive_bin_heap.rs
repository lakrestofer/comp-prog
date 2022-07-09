use crate::Heap;
use std::collections::VecDeque;

/// a simple binary heap that implements the methods of the Heap interface
struct NaiveBinaryHeap<T: PartialOrd> {
    vec: Vec<T>,
    size: usize,
}

impl<T: PartialOrd> NaiveBinaryHeap<T> {
    /// checks if the last element is in order, and if not flips it with its parent. It then checks if the parent is in order
    fn restore_order(&mut self) {
        // we only need to do anything if the heap is non-empty
        if !self.vec.is_empty() {
            let mut last_index = self.vec.len() - 1;
            loop {
                if last_index > 0 {
                    // last index has a parent
                    let parent_index = (last_index - 1) / 2;
                    if self.vec[parent_index] <= self.vec[last_index] {
                        // the heap order is fullfilled and we are done
                        break;
                    } else {
                        // the parent is larger than the child
                        // swap the elements and set the last_index to parent_index
                        self.vec.swap(parent_index, last_index);
                        last_index = parent_index;
                    }
                } else {
                    // there is no other node to compare order with, we are done.
                    break;
                }
            }
        }
    }
}

impl<T: PartialOrd> Heap<T> for NaiveBinaryHeap<T> {
    fn insert(&mut self, elem: T) {
        // we push the new elem to the end of the vec
        self.vec.push(elem);
        self.size += 1;
        self.restore_order()
    }

    fn peek(&self) -> Option<&T> {
        self.vec.first()
    }

    fn pop(&mut self) -> Option<T> {
        if self.vec.len() == 1 {
            self.vec.pop()
        } else {
            let last_index = self.vec.len() - 1;
            // we swap the min element and last place
            self.vec.swap()
        }
    }

    fn replace(&mut self, elem: T) -> Option<T> {
        todo!()
    }

    fn new() -> Self {
        Self {
            vec: Vec::new(),
            size: 0,
        }
    }

    fn heapify(list: Vec<T>) -> Self {
        todo!()
    }

    fn merge(other: Self) -> Self {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn capacity(&self) -> usize {
        todo!()
    }
}
