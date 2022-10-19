//! Thread-safe queue implemented rust.
//!
//! # Example
//! ```
//! use std::thread;
//! use std::time;
//!
//! use rueue::{FifoQueue, Queue};
//!
//! let mut queue = FifoQueue::new(None);
//!
//! let mut q1 = queue.clone();
//! let th1 = thread::spawn(move || {
//!     for i in 0..3 {
//!         q1.put_wait(i, time::Duration::from_millis(100)).unwrap();
//!         thread::sleep(time::Duration::from_millis(10));
//!     }
//! });
//!
//! let mut q2 = queue.clone();
//! let th2 = thread::spawn(move || {
//!     if let Ok(item) = q2.get_wait(time::Duration::from_millis(0)) {
//!         assert_eq!(item, 0);
//!     }
//!     if let Ok(item) = q2.get_wait(time::Duration::from_millis(0)) {
//!         assert_eq!(item, 1);
//!     }
//!     if let Ok(item) = q2.get_wait(time::Duration::from_millis(0)) {
//!         assert_eq!(item, 2);
//!     }
//!     assert!(q2.is_empty());
//! });
//! th1.join().unwrap();
//! th2.join().unwrap();
//! ```

mod queue;
pub use queue::{PutError, Queue, QueueError};

mod fifo_queue;
pub use fifo_queue::FifoQueue;

mod lifo_queue;
pub use lifo_queue::LifoQueue;

mod priority_queue;
pub use priority_queue::{PrioritizedItem, PriorityQueue};
