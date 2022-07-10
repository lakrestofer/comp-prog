//! A simple implementation of a binary min heap using a vec.

use crate::Heap;
struct NaiveBinHeap<T: PartialOrd> {
    nodes: Vec<T>,
}

impl<T: PartialOrd> NaiveBinHeap<T> {
    /// shifts down the element with index {index} if neccessary
    fn down(&mut self) {
        let max_index = self.nodes.len() - 1;
        let mut parent = 0;

        loop {
            let first_child = parent * 2 + 1;
            let second_child = parent * 2 + 2;

            // base case
            if first_child > max_index {
                break;
            }

            let mut child_to_swap = first_child;

            // if the second child exist and if is is lesser than the first child we swap with that instead
            if second_child <= max_index && self.nodes[first_child] >= self.nodes[second_child] {
                child_to_swap = second_child;
            }

            // we only do the swap if the actually need to
            if self.nodes[parent] > self.nodes[child_to_swap] {
                self.nodes.swap(parent, child_to_swap);
                parent = child_to_swap;
            } else {
                break;
            }
        }
    }

    fn up(&mut self) {
        let mut child = self.nodes.len() - 1;
        loop {
            // 0 1 2 3 4 5 6
            let parent = child - 1 / 2;
            if self.nodes[parent] > self.nodes[child] {
                self.nodes.swap(parent, child);
                child = parent;
            } else {
                break;
            }
        }
    }
}

impl<T: PartialOrd> Heap<T> for NaiveBinHeap<T> {
    fn insert(&mut self, elem: T) {
        // insert new element
        self.nodes.push(elem);
        // and then we restore the heap order
        self.up();
    }

    fn peek(&self) -> Option<&T> {
        self.nodes.first()
    }

    fn pop(&mut self) -> Option<T> {
        // we swap the first and the last element in the vec
        // then we can simply pop
        let max_index = self.nodes.len() - 1;
        self.nodes.swap(0, max_index);
        let res = self.nodes.pop();
        // then we restore the order
        self.down();
        res
    }

    fn replace(&mut self, elem: T) -> Option<T> {
        todo!()
    }

    fn new() -> Self {
        todo!()
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
