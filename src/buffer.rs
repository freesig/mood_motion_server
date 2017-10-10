use std;
use std::collections::VecDeque;

pub struct Buffer{
    vals: VecDeque<f32>,
    size: usize,
}

impl Buffer{
    pub fn new(size: usize) -> Self {
        let vals: VecDeque<f32> = VecDeque::with_capacity(size);
        Buffer{ vals, size } 
    }
    
    pub fn new_fill(v: f32, size: usize) -> Self {
        let mut vals: VecDeque<f32> = VecDeque::with_capacity(size);
        vals.extend( std::iter::repeat(v).take(size) );
        Buffer{ vals, size } 
    }
    
    pub fn add(&mut self, val: f32) {
        self.vals.push_back(val);
        if self.vals.len() >= self.size {
            self.vals.pop_front();
        }
    }

    pub fn cap(&self) -> usize {
        self.size
    }

    pub fn last(&self) -> Option<f32>{
        match self.vals.back(){
            Some(v) => Some(*v),
            None => None,
        }
    }
}

pub fn average(b: & Buffer) -> f32 {
    let mut total = 0.0;
    for v in & b.vals {
        total += *v;
    }
    total / b.vals.len() as f32
}

pub fn new_buff_size(to: usize, b: &mut Buffer) {
    if b.size > to {
        b.vals.truncate(to);
    }else if b.size < to {
        b.vals.reserve(to - b.size);
    }
    b.size = to;
}
