use std::cmp::{Ord, Ordering};
use std::collections::BinaryHeap;

use crate::queue::*;

#[derive(Debug)]
pub struct PrioritizedItem<T, P>(pub T, pub P);

impl<T, P: Ord> Eq for PrioritizedItem<T, P> {}

impl<T, P: Ord> PartialEq<Self> for PrioritizedItem<T, P> {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl<T, P: Ord> PartialOrd<Self> for PrioritizedItem<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl<T, P: Ord> Ord for PrioritizedItem<T, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl<T, P: Ord> BasicArray<PrioritizedItem<T, P>> for BinaryHeap<PrioritizedItem<T, P>> {
    fn new(maxsize: Option<usize>) -> Self {
        match maxsize {
            None => BinaryHeap::new(),
            Some(s) => BinaryHeap::with_capacity(s),
        }
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&mut self) -> Option<PrioritizedItem<T, P>> {
        self.pop()
    }

    fn put(&mut self, value: PrioritizedItem<T, P>) {
        self.push(value)
    }
}

/// Queue with a priority.
///
/// # Example
/// ```
/// use rueue::{PriorityQueue, PrioritizedItem, Queue};
///
/// let mut queue = PriorityQueue::new(None);
///
/// queue.put(PrioritizedItem(1, 10)).unwrap();
/// queue.put(PrioritizedItem(2, 8)).unwrap();
/// queue.put(PrioritizedItem(3, 9)).unwrap();
///
/// let first_item = queue.get().unwrap();
/// assert_eq!(first_item.0, 1);
/// assert_eq!(first_item.1, 10);
///
/// let second_item = queue.get().unwrap();
/// assert_eq!(second_item.0, 3);
/// assert_eq!(second_item.1, 9);
///
/// let third_item = queue.get().unwrap();
/// assert_eq!(third_item.0, 2);
/// assert_eq!(third_item.1, 8);
/// ```
pub type PriorityQueue<T, P> = BasicQueue<BinaryHeap<PrioritizedItem<T, P>>, PrioritizedItem<T, P>>;
