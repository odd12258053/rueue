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

pub type FiFoQueue<T> = BasicQueue<VecDeque<T>, T>;
