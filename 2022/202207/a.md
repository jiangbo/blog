use std::fmt::Debug;

fn main() {
    let mut heap = MyHeap::default();
    (0..10).for_each(|e| heap.insert(e));

    println!("{:?}", heap);
    println!("{:?}", heap.max());
    while let Some(max) = heap.get_max() {
        println!("{:?}", max);
    }

    let vec = (0..10).collect();
    heap = MyHeap::new(vec);
    println!("{:?}", heap);
    println!("{:?}", heap.max());
    while let Some(max) = heap.get_max() {
        println!("{:?}", max);
    }
}

#[derive(Debug, Default)]
pub struct MyHeap<T> {
    vec: Vec<T>,
}

impl<T: Ord> MyHeap<T> {
    fn get(&self, idx: usize) -> Option<&T> {
        self.vec.get(idx)
    }

    fn new(vec: Vec<T>) -> Self {
        let mut heap = Self { vec };
        let mut idx = heap.vec.len() / 2;
        while idx > 0 {
            idx -= 1;
            heap.down(idx);
        }
        heap
    }

    fn down(&mut self, mut idx: usize) {
        loop {
            let mut max = 2 * idx + 1;
            max = match self.get(max) > self.get(max + 1) {
                true => max,
                false => max + 1,
            };
            match self.get(max) > self.get(idx) {
                true => self.vec.swap(idx, max),
                false => return,
            };
            idx = max;
        }
    }
}

impl<T: Ord> Heap<T> for MyHeap<T> {
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
        let result = Some(&mut self.vec)
            .filter(|vec| !vec.is_empty())
            .map(|v| v.swap_remove(0));
        self.down(0);
        result
    }
}

trait Heap<T> {
    fn insert(&mut self, value: T);
    fn max(&self) -> Option<&T>;
    fn get_max(&mut self) -> Option<T>;
}
