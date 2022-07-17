pub mod naive_bin_heap;
/// generic heap trait that defines all the methods that a heap should implement
pub trait Heap<T: PartialOrd + PartialEq> {
    // insert an element into the heap
    fn insert(&mut self, elem: T);
    /// gives a reference to the min/max element of the heap
    fn peek(&self) -> Option<&T>;
    /// returns the min/max element of the heap
    fn pop(&mut self) -> Option<T>;
    /// pops and inserts simultaneously. More efficient than performing the operations sequentially.
    fn replace(&mut self, elem: T) -> Option<T>;
    /// create a new 'empty' heap
    fn new() -> Self;
    /// creates a new heap from the contents of a list
    fn heapify(list: Vec<T>) -> Self;
}
