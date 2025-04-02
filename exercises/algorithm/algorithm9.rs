/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // Add the value at the end of the heap
        self.items.push(value);
        self.count += 1;

        // Get the index of the inserted value
        let mut idx = self.count;

        // Heapify up: move the newly added value up as needed
        // to maintain the heap property
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);

            // If the parent has higher priority (according to the comparator)
            // than the current item, we're done
            if (self.comparator)(&self.items[parent_idx], &self.items[idx]) {
                break;
            }

            // Otherwise, swap the current item with its parent
            self.items.swap(parent_idx, idx);

            // Move up to the parent position
            idx = parent_idx;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // Get indices of left and right children
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // If the right child exists and has higher priority than the left child
        // according to the comparator, return the right child's index
        if right_idx <= self.count &&
            (self.comparator)(&self.items[right_idx], &self.items[left_idx]) {
            return right_idx;
        }

        // Otherwise, return the left child's index
        left_idx
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // If the heap is empty, return None
        if self.is_empty() {
            return None;
        }

        // The highest priority element is at index 1
        // Swap it with the last element
        self.items.swap(1, self.count);

        // Remove the highest priority element (now at the last position)
        let result = self.items.pop().unwrap();
        self.count -= 1;

        // If the heap is not empty after removing the element,
        // we need to heapify down to maintain the heap property
        if !self.is_empty() {
            let mut idx = 1;

            // Continue heapifying down as long as the current node has children
            // and the heap property is violated
            while self.children_present(idx) {
                let smallest_child = self.smallest_child_idx(idx);

                // If the current node has higher priority than its smallest child,
                // the heap property is satisfied
                if (self.comparator)(&self.items[idx], &self.items[smallest_child]) {
                    break;
                }

                // Otherwise, swap the current node with its smallest child
                self.items.swap(idx, smallest_child);

                // Move down to the position of the smallest child
                idx = smallest_child;
            }
        }

        // Return the previously highest priority element
        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}