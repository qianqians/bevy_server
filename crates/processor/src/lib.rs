#![feature(async_closure)]
#![feature(future_join)]

use std::{collections::VecDeque, future::Future, future::join};

pub struct Processor<T> {
    que : VecDeque<Box<T> >
}

impl <T> Processor<T> {
    pub fn new() -> Processor<T> {
        Processor {
            que: VecDeque::new()
        }
    }

    pub fn enque(&mut self, t:T) {
        self.que.push_back(Box::new(t));
    }

    pub fn process(&mut self, f:fn(T)) {
        loop {
            let p = self.que.pop_back();
            match p {
                None => break,
                Some(t) => f(*t)
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