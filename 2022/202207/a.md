use std::fmt::Debug;

use crate::Heap;

#[derive(Debug)]
pub struct MyHeap<T> {
    vec: Vec<T>,
}

impl<T: Ord> MyHeap<T> {
    fn get(&self, idx: usize) -> Option<&T> {
        self.vec.get(idx)
    }
}

impl<T: Ord> Heap<T> for MyHeap<T> {
    fn new() -> Self {
        MyHeap { vec: Vec::new() }
    }

    fn insert(&mut self, value: T) {
        self.vec.push(value);
        let mut idx = self.vec.len() - 1;
        let mut pdx;
        while idx > 0 {
            pdx = (idx - 1) / 2;
            if self.get(idx) > self.get(pdx) {
                self.vec.swap(idx, pdx);
                idx = pdx;
            }
        }
    }

    fn max(&self) -> Option<&T> {
        self.vec.first()
    }

    fn get_max(&mut self) -> Option<T> {
        if self.vec.is_empty() {
            return None;
        }
        let result = self.vec.swap_remove(0);
        let mut idx = 0;
        loop {
            let mut max = 2 * idx + 1;
            max = match self.get(max) > self.get(max + 1) {
                true => max,
                false => max + 1,
            };
            match self.get(max) > self.get(idx) {
                true => self.vec.swap(idx, max),
                false => return Some(result),
            };
            idx = max;
        }
    }
}


pub trait Heap<T> {
    fn new() -> Self;
    fn insert(&mut self, value: T);
    fn max(&self) -> Option<&T>;
    fn get_max(&mut self) -> Option<T>;
}

