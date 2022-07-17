//! A simple implementation of a binary min heap using a vec.

use crate::Heap;
pub struct NaiveBinHeap<T: PartialOrd> {
    nodes: Vec<T>,
}

impl<T: PartialOrd> NaiveBinHeap<T> {
    /// shifts down the element with index {index} if neccessary
    fn down(&mut self) {
        let len = self.nodes.len();
        if len >= 2 {
            let max_index = len - 1;
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
                if second_child <= max_index && self.nodes[first_child] >= self.nodes[second_child]
                {
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
    }

    fn up(&mut self) {
        let len = self.nodes.len();
        let mut child = len - 1;
        if len >= 2 {
            loop {
                if child == 0 {
                    break;
                }

                let parent = (child - 1) / 2;
                if self.nodes[parent] > self.nodes[child] {
                    self.nodes.swap(parent, child);
                    child = parent;
                } else {
                    break;
                }
            }
        }
    }

    fn get_list(self) -> Vec<T> {
        self.nodes
    }

    fn get_list_ref(&self) -> &Vec<T> {
        &self.nodes
    }
}

impl<T: PartialOrd> Heap<T> for NaiveBinHeap<T> {
    /// inserts new element in the list
    fn insert(&mut self, elem: T) {
        // insert new element
        self.nodes.push(elem);
        // and then we restore the heap order
        self.up();
    }

    /// gets reference to the top of the pq
    fn peek(&self) -> Option<&T> {
        self.nodes.first()
    }

    /// gets element on top of the pq
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

    /// does a simultanious insertion and pop from the heap. This is more efficient since we only need to do the reordering once
    fn replace(&mut self, elem: T) -> Option<T> {
        self.nodes.push(elem);
        let last = self.nodes.len() - 1;
        self.nodes.swap(0, last);
        let res = self.nodes.pop();
        // reorder the topmost element
        self.down();
        res
    }

    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn heapify(mut list: Vec<T>) -> Self {
        let len = list.len();
        if len > 1 {
            let max_index = len - 1;
            let last_parent = max_index - 1 / 2;
            let mut current_parent = last_parent;

            while current_parent != 0 {
                let mut parent = current_parent;
                loop {
                    let first_child = parent * 2 + 1;
                    let second_child = parent * 2 + 2;

                    if first_child > max_index {
                        break;
                    }

                    let mut child_to_swap = first_child;
                    // if the second child exist and if is is lesser than the first child we swap with that instead
                    if second_child <= max_index && list[first_child] >= list[second_child] {
                        child_to_swap = second_child;
                    }
                    if list[parent] > list[child_to_swap] {
                        list.swap(parent, child_to_swap);
                        parent = child_to_swap;
                    } else {
                        break;
                    }
                }
                current_parent -= 1;
            }
        }
        Self { nodes: list }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn insert_and_pop() {
        let mut heap = NaiveBinHeap::new();
        heap.insert(4);
        heap.insert(3);
        heap.insert(7);
        heap.insert(1);
        assert!(heap.pop() == Some(1));
        assert!(heap.pop() == Some(3));
        assert!(heap.pop() == Some(4));
        assert!(heap.pop() == Some(7));
    }

    #[test]
    fn heapify_test() {
        let vec = vec![4, 3, 7, 1];
        let mut heap = NaiveBinHeap::heapify(vec);
        println!("heap: {:?}", heap.get_list_ref());
        assert!(heap.pop() == Some(1));
        assert!(heap.pop() == Some(3));
        assert!(heap.pop() == Some(4));
        assert!(heap.pop() == Some(7));
    }
}
