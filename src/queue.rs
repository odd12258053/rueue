use std::marker::PhantomData;
use std::sync::{Arc, Condvar, Mutex};
use std::time;

#[derive(Debug)]
pub enum QueueError {
    Full,
    Empty,
}

#[derive(Debug)]
pub struct PutError<T>(T, QueueError);

pub trait Queue<T> {
    ///
    /// # Example
    /// ```
    /// use rueue::{FifoQueue, Queue};
    ///
    /// let mut queue = FifoQueue::new(None);
    /// queue.put(1).unwrap();
    ///
    /// assert_eq!(queue.len(), 1);
    /// ```
    fn len(&self) -> usize;

    ///
    /// # Example
    /// ```
    /// use rueue::{FifoQueue, Queue};
    ///
    /// let mut queue = FifoQueue::new(None);
    ///
    /// assert_eq!(queue.is_empty(), true);
    ///
    /// queue.put(1).unwrap();
    /// assert_eq!(queue.is_empty(), false);
    /// ```
    fn is_empty(&self) -> bool;

    ///
    /// # Example
    /// ```
    /// use rueue::{FifoQueue, Queue};
    ///
    /// let mut queue = FifoQueue::new(Some(1));
    ///
    /// assert_eq!(queue.is_full(), false);
    ///
    /// queue.put(1).unwrap();
    /// assert_eq!(queue.is_full(), true);
    /// ```
    fn is_full(&self) -> bool;

    ///
    /// # Example
    /// ```
    /// use rueue::{FifoQueue, Queue};
    ///
    /// let mut queue = FifoQueue::new(None);
    ///
    /// queue.put(1).unwrap();
    /// let item = queue.get().unwrap();
    /// assert_eq!(item, 1);
    /// ```
    fn get(&mut self) -> Result<T, QueueError>;

    ///
    /// # Example
    /// ```
    /// use std::time;
    /// use rueue::{FifoQueue, Queue};
    ///
    /// let mut queue = FifoQueue::new(None);
    ///
    /// queue.put(1).unwrap();
    /// let item = queue.get_wait(time::Duration::from_millis(1000)).unwrap();
    /// assert_eq!(item, 1);
    /// ```
    fn get_wait(&mut self, timeout: time::Duration) -> Result<T, QueueError>;

    ///
    /// # Example
    /// ```
    /// use rueue::{FifoQueue, Queue};
    ///
    /// let mut queue = FifoQueue::new(None);
    ///
    /// queue.put(1).unwrap();
    /// let item = queue.get().unwrap();
    /// assert_eq!(item, 1);
    /// ```
    fn put(&mut self, value: T) -> Result<(), PutError<T>>;

    ///
    /// # Example
    /// ```
    /// use std::time;
    /// use rueue::{FifoQueue, Queue};
    ///
    /// let mut queue = FifoQueue::new(None);
    ///
    /// queue.put_wait(1, time::Duration::from_millis(1000)).unwrap();
    /// let item = queue.get().unwrap();
    /// assert_eq!(item, 1);
    /// ```
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

pub struct BaseQueue<Q, T> {
    pub(crate) inner: Arc<QueueInner<Q, T>>,
}

impl<Q: BasicArray<T>, T> BaseQueue<Q, T> {
    pub fn new(maxsize: Option<usize>) -> Self {
        Self {
            inner: Arc::new(QueueInner::new(maxsize)),
        }
    }
}

impl<Q: BasicArray<T>, T> Queue<T> for BaseQueue<Q, T> {
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

impl<Q, T> Clone for BaseQueue<Q, T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
