use std::cmp::{Ord, Ordering};
use std::collections::BinaryHeap;

use crate::queue::*;

#[derive(Debug)]
pub struct PrioritizedItem<T, P> {
    pub item: T,
    pub priority: P,
}

impl<T, P> PrioritizedItem<T, P> {
    pub fn new(item: T, priority: P) -> Self {
        Self { item, priority }
    }
}

impl<T, P: Ord> Eq for PrioritizedItem<T, P> {}

impl<T, P: Ord> PartialEq<Self> for PrioritizedItem<T, P> {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl<T, P: Ord> PartialOrd<Self> for PrioritizedItem<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T, P: Ord> Ord for PrioritizedItem<T, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
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

pub type PriorityQueue<T, P> = BasicQueue<BinaryHeap<PrioritizedItem<T, P>>, PrioritizedItem<T, P>>;
