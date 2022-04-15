# 【Rust】二叉堆-排序

## 环境

- Time 2022-04-15
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。

### 特点

二叉堆的极值在最前面，可能是最大值或者最小值，又叫大顶堆或者小顶堆。
标准库中已经内置了一个二叉堆（BinaryHeap），这里只做练习使用。
堆的实现可以参考建堆的过程或者附录中的源码。

## 示例

### 实现

```rust
fn heap_sort(data: &mut [i32]) {
    MyHeap::heapify(data);
    for end in (1..data.len()).rev() {
        data.swap(0, end);
        MyHeap::down(&mut data[..end], 0)
    }
}
```

### 空元素

```rust
#[test]
fn test_empty() {
    let mut data = vec![];
    heap_sort(&mut data);
    assert_eq!(data, vec![]);
}
```

### 单元素

```rust
#[test]
fn test_single() {
    let mut data = vec![44];
    heap_sort(&mut data);
    assert_eq!(data, vec![44]);
}
```

### 多元素

```rust
#[test]
fn test_multi() {
    let mut data = vec![44, 55, 33, 22, 0, -44];
    heap_sort(&mut data);
    assert_eq!(data, vec![-44, 0, 22, 33, 44, 55]);
}
```

## 总结

使用自定义的二叉堆，实现了堆排序。

## 附录

### 源码

```rust
fn main() {
    let vec = (0..10).rev().collect();
    let mut heap = MyHeap::new(vec);
    println!("{:?}", heap.peek());
    (0..10).for_each(|e| heap.push(e));
    println!("{heap:?}");
    while let Some(max) = heap.pop() {
        println!("{:?}", max);
    }
}
fn heap_sort(data: &mut [i32]) {
    MyHeap::heapify(data);
    for end in (1..data.len()).rev() {
        data.swap(0, end);
        MyHeap::down(&mut data[..end], 0)
    }
}

#[test]
fn test_empty() {
    let mut data = vec![];
    heap_sort(&mut data);
    assert_eq!(data, vec![]);
}
#[test]
fn test_single() {
    let mut data = vec![44];
    heap_sort(&mut data);
    assert_eq!(data, vec![44]);
}
#[test]
fn test_multi() {
    let mut data = vec![44, 55, 33, 22, 0, -44];
    heap_sort(&mut data);
    assert_eq!(data, vec![-44, 0, 22, 33, 44, 55]);
}

trait Heap<T> {
    /// 插入一个元素
    fn push(&mut self, value: T);
    /// 查询堆顶元素
    fn peek(&self) -> Option<&T>;
    /// 弹出堆顶元素
    fn pop(&mut self) -> Option<T>;
}

#[derive(Debug, Default)]
pub struct MyHeap<T> {
    vec: Vec<T>,
}

impl<T: Ord> MyHeap<T> {
    fn new(mut vec: Vec<T>) -> Self {
        Self::heapify(&mut vec);
        Self { vec }
    }
    fn heapify(vec: &mut [T]) {
        let mut idx = vec.len() / 2;
        while idx > 0 {
            idx -= 1;
            Self::down(vec, idx);
        }
    }
    fn get(&self, idx: usize) -> Option<&T> {
        self.vec.get(idx)
    }
    fn down(vec: &mut [T], mut idx: usize) {
        loop {
            let mut max = 2 * idx + 1;
            max = match vec.get(max) > vec.get(max + 1) {
                true => max,
                false => max + 1,
            };
            match vec.get(max) > vec.get(idx) {
                true => vec.swap(idx, max),
                false => return,
            };
            idx = max;
        }
    }
}

impl<T: Ord> Heap<T> for MyHeap<T> {
    fn push(&mut self, value: T) {
        self.vec.push(value);
        let mut idx = self.vec.len() - 1;
        while idx > 0 {
            let pdx = (idx - 1) / 2;
            match self.get(idx) > self.get(pdx) {
                true => self.vec.swap(idx, pdx),
                false => return,
            }
            idx = pdx;
        }
    }

    fn peek(&self) -> Option<&T> {
        self.vec.first()
    }

    fn pop(&mut self) -> Option<T> {
        let result = Some(&mut self.vec)
            .filter(|vec| !vec.is_empty())
            .map(|v| v.swap_remove(0));
        Self::down(self.vec.as_mut(), 0);
        result
    }
}
```
