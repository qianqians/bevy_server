use std::{collections::VecDeque};
use std::sync::Mutex;

pub struct Queue<T> {
    que : Mutex<VecDeque<T>>
}

impl <T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            que: Mutex::new(VecDeque::new())
        }
    }

    pub fn enque(&mut self, t:T) {
        let mut que = self.que.lock().unwrap();
        que.push_back(t);
    }

    pub fn deque(&mut self) -> Option<T> {
        let mut que = self.que.lock().unwrap();
        que.pop_back()
    }
}