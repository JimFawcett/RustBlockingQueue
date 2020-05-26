/////////////////////////////////////////////////////////////
// rust_blocking_queue::lib.rs - BlockingQueue             //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 19 May 2020  //
/////////////////////////////////////////////////////////////
/*
   This is a BlockingQueue abstraction.  To be shared between
   threads, without using unsafe code, any abstraction must
   be composed only of Mutexes and Condvars or a struct or
   tuple with only those members.

   That means that the blocking queue must hold its native
   queue in a Mutex, as shown below.
   
   There is another alternative, based on Rust channels, which 
   are essentially blocking queues.
*/
#![allow(dead_code)]
use std::sync::*;
use std::collections::*;

#[derive(Debug)]
/// Thread-safe queue that blocks de_q on empty
pub struct BlockingQueue<T> {
    q: Mutex<VecDeque<T>>,
    cv: Condvar,
}
impl<T> BlockingQueue<T> {
    /// Create empty blocking queue
    pub fn new() -> Self {
        Self {
            q: Mutex::new(VecDeque::new()),
            cv: Condvar::new(),
        }
    }
    /// push input on back of queue
    /// - unrecoverable if lock fails so just unwrap
    pub fn en_q(&self, t:T) {
        let mut lq = self.q.lock().unwrap();
        lq.push_back(t);
        self.cv.notify_one();
    }
    /// pop element from front of queue
    /// - unrecoverable if lock fails so just unwrap
    /// - same for condition variable
    pub fn de_q(&self) -> T {
        let mut lq = self.q.lock().unwrap();
        while lq.len() == 0 {
            lq = self.cv.wait(lq).unwrap();
        }
        lq.pop_front().unwrap()
    }
    /// return number of elements in queue
    pub fn len(&self) -> usize {
        self.q.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bq_len() {
        let bq = BlockingQueue::<f64>::new();
        assert_eq!(bq.len(), 0);
    }
    #[test]
    fn bq_en_queue() {
        let bq = BlockingQueue::<f64>::new();
        bq.en_q(3.5);
        assert_eq!(bq.len(), 1);
    }
    #[test]
    fn bq_de_queue() {
        let bq = BlockingQueue::<f64>::new();
        bq.en_q(3.5);
        assert_eq!(bq.de_q(), 3.5);
        assert_eq!(bq.len(), 0);
    }
}
