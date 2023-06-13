use std::{collections::VecDeque};
use std::sync::Mutex;

pub struct Processor<T> {
    que : Mutex<VecDeque<T>>
}

impl <T> Processor<T> {
    pub fn new() -> Processor<T> {
        Processor {
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

struct TestStruct {
    i: i32,
    str: String
}

fn test() {
    let mut p = Processor::<TestStruct>::new();
    p.enque(TestStruct{
        i: 1,
        str: "qianqians".to_string()
    });
    let _ = p.deque();
}