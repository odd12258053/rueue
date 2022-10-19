use std::collections::VecDeque;

use crate::queue::*;

impl<T> BasicArray<T> for VecDeque<T> {
    fn new(maxsize: Option<usize>) -> Self {
        match maxsize {
            None => VecDeque::new(),
            Some(s) => VecDeque::with_capacity(s),
        }
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&mut self) -> Option<T> {
        self.pop_front()
    }

    fn put(&mut self, value: T) {
        self.push_back(value)
    }
}

/// Fifo (First in, First out) Queue.
///
/// # Example
/// ```
/// use rueue::{FifoQueue, Queue};
///
/// let mut queue = FifoQueue::new(None);
///
/// queue.put(1).unwrap();
/// queue.put(2).unwrap();
/// queue.put(3).unwrap();
///
/// let first_item = queue.get().unwrap();
/// assert_eq!(first_item, 1);
///
/// let second_item = queue.get().unwrap();
/// assert_eq!(second_item, 2);
///
/// let third_item = queue.get().unwrap();
/// assert_eq!(third_item, 3);
/// ```
pub type FifoQueue<T> = BaseQueue<VecDeque<T>, T>;
