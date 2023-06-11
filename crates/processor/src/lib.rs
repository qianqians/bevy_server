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

    pub fn process(&mut self, f:fn(T)) {
        loop {
            let mut que = self.que.lock().unwrap();
            let p = que.pop_back();
            match p {
                None => break,
                Some(t) => f(t)
            }
        }
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
    p.process(|t|{
        let num = t.i;
        let _str = t.str;
        print!("str:{0}, num:{1}", _str, num);
    });
}