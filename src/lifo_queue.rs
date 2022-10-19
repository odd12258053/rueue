use crate::queue::*;

impl<T> BasicArray<T> for Vec<T> {
    fn new(maxsize: Option<usize>) -> Self {
        match maxsize {
            None => Vec::new(),
            Some(s) => Vec::with_capacity(s),
        }
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&mut self) -> Option<T> {
        self.pop()
    }

    fn put(&mut self, value: T) {
        self.push(value)
    }
}

/// Lifo (Last in, First out) Queue.
///
/// # Example
/// ```
/// use rueue::{LifoQueue, Queue};
///
/// let mut queue = LifoQueue::new(None);
///
/// queue.put(1).unwrap();
/// queue.put(2).unwrap();
/// queue.put(3).unwrap();
///
/// let first_item = queue.get().unwrap();
/// assert_eq!(first_item, 3);
///
/// let second_item = queue.get().unwrap();
/// assert_eq!(second_item, 2);
///
/// let third_item = queue.get().unwrap();
/// assert_eq!(third_item, 1);
/// ```
pub type LifoQueue<T> = BasicQueue<Vec<T>, T>;
