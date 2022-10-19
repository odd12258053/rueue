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

pub type LiFoQueue<T> = BasicQueue<Vec<T>, T>;
