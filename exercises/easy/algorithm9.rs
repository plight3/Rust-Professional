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
        self.items.push(value);
        self.count += 1;
        let mut cur_idx = self.count;
        while cur_idx != 1 {
            let parent_idx = self.parent_idx(cur_idx);
            if (self.comparator)(&self.items[cur_idx], &self.items[parent_idx]) {
                self.items.swap(cur_idx, parent_idx);
                cur_idx = parent_idx;
                continue;
            }
            break;
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
        //TODO
		0
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
        if !self.is_empty() {
            self.items.swap(1, self.count);
            let item = self.items.remove(self.count);
            self.count -= 1;
            if self.count > 1 {
                let mut cur_idx = 1;
                while self.children_present(cur_idx) {
                    let left_idx = self.left_child_idx(cur_idx);
                    let right_idx = self.right_child_idx(cur_idx);
                    if right_idx > self.count {
                        if !(self.comparator)(&self.items[cur_idx], &self.items[left_idx]) {
                            self.items.swap(cur_idx, left_idx);
                            cur_idx = left_idx;
                        }
                        break;
                    } else {
                        if !(self.comparator)(&self.items[cur_idx], &self.items[left_idx]) ||
                         !(self.comparator)(&self.items[cur_idx], &self.items[right_idx]) {
                            if (self.comparator)(&self.items[left_idx], &self.items[right_idx]){
                                self.items.swap(cur_idx, left_idx);
                                cur_idx = left_idx;
                            }
                            else {
                                self.items.swap(cur_idx, right_idx);
                                cur_idx = right_idx;
                            }
                         }
                         break;
                    }

                }
            }
            Some(item)
        } else {
            None
        }
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