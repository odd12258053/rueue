use std::marker::PhantomData;
use std::sync::{Arc, Condvar, Mutex};
use std::time;

#[derive(Debug)]
pub enum QueueError {
    Full,
    Empty,
}

pub struct PutError<T>(T, QueueError);

pub trait Queue<T> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
    fn get(&mut self) -> Result<T, QueueError>;
    fn get_wait(&mut self, timeout: time::Duration) -> Result<T, QueueError>;
    fn put(&mut self, value: T) -> Result<(), PutError<T>>;
    fn put_wait(&mut self, value: T, timeout: time::Duration) -> Result<(), PutError<T>>;
}

pub trait BasicArray<T> {
    fn new(maxsize: Option<usize>) -> Self;
    fn len(&self) -> usize;
    fn get(&mut self) -> Option<T>;
    fn put(&mut self, value: T);
}

pub(crate) struct QueueInner<Q, T> {
    _item: PhantomData<T>,
    pub(crate) queue: Mutex<Q>,
    pub(crate) maxsize: Option<usize>,
    pub(crate) pending: Mutex<()>,
    pub(crate) not_empty: Condvar,
    pub(crate) not_full: Condvar,
}

impl<Q: BasicArray<T>, T> QueueInner<Q, T> {
    pub fn new(maxsize: Option<usize>) -> Self {
        Self {
            _item: PhantomData,
            queue: Mutex::new(Q::new(maxsize)),
            maxsize,
            pending: Mutex::new(()),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }
}

pub struct BasicQueue<Q, T> {
    pub(crate) inner: Arc<QueueInner<Q, T>>,
}

impl<Q: BasicArray<T>, T> BasicQueue<Q, T> {
    pub fn new(maxsize: Option<usize>) -> Self {
        Self {
            inner: Arc::new(QueueInner::new(maxsize)),
        }
    }
}

impl<Q: BasicArray<T>, T> Queue<T> for BasicQueue<Q, T> {
    fn len(&self) -> usize {
        self.inner.queue.lock().unwrap().len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_full(&self) -> bool {
        Some(self.len()) == self.inner.maxsize
    }

    fn get(&mut self) -> Result<T, QueueError> {
        if let Some(value) = self.inner.queue.lock().unwrap().get() {
            self.inner.not_full.notify_one();
            Ok(value)
        } else {
            Err(QueueError::Empty)
        }
    }

    fn get_wait(&mut self, timeout: time::Duration) -> Result<T, QueueError> {
        if timeout.is_zero() {
            while self.is_empty() {
                let _pending = self
                    .inner
                    .not_empty
                    .wait(self.inner.pending.lock().unwrap())
                    .unwrap();
            }
        } else {
            let timestamp = time::SystemTime::now();
            let mut remaining = timeout;
            while self.is_empty() {
                let ret = self
                    .inner
                    .not_empty
                    .wait_timeout(self.inner.pending.lock().unwrap(), remaining)
                    .unwrap();
                if ret.1.timed_out() {
                    return Err(QueueError::Empty);
                }
                let elapsed = timestamp.elapsed().unwrap();
                if elapsed >= timeout {
                    return Err(QueueError::Empty);
                }
                remaining = timeout - elapsed;
            }
        }
        self.get()
    }

    fn put(&mut self, value: T) -> Result<(), PutError<T>> {
        let mut queue = self.inner.queue.lock().unwrap();
        if Some(queue.len()) == self.inner.maxsize {
            return Err(PutError(value, QueueError::Full));
        }
        queue.put(value);
        self.inner.not_empty.notify_one();
        Ok(())
    }

    fn put_wait(&mut self, value: T, timeout: time::Duration) -> Result<(), PutError<T>> {
        if timeout.is_zero() {
            while self.is_full() {
                let _pending = self
                    .inner
                    .not_full
                    .wait(self.inner.pending.lock().unwrap())
                    .unwrap();
            }
        } else {
            let timestamp = time::SystemTime::now();
            let mut remaining = timeout;
            while self.is_full() {
                let ret = self
                    .inner
                    .not_full
                    .wait_timeout(self.inner.pending.lock().unwrap(), remaining)
                    .unwrap();
                if ret.1.timed_out() {
                    return Err(PutError(value, QueueError::Full));
                }
                let elapsed = timestamp.elapsed().unwrap();
                if elapsed >= timeout {
                    return Err(PutError(value, QueueError::Full));
                }
                remaining = timeout - elapsed;
            }
        }
        self.put(value)
    }
}

impl<Q, T> Clone for BasicQueue<Q, T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
