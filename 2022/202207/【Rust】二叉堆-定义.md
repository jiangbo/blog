# 【Rust】二叉堆-定义

## 环境

- Time 2022-04-15
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。

### 特点

二叉堆的极值在最前面，可能是最大值或者最小值，又叫大顶堆或者小顶堆。
标准库中已经内置了一个二叉堆（BinaryHeap），这里只做练习使用。

## 示例

### 操作定义

```rust
trait Heap<T> {
    /// 插入一个元素
    fn push(&mut self, value: T);
    /// 查询堆顶元素
    fn peek(&self) -> Option<&T>;
    /// 弹出堆顶元素
    fn pop(&mut self) -> Option<T>;
}
```

### 结构定义

```rust
#[derive(Debug, Default)]
pub struct MyHeap<T> {
    vec: Vec<T>,
}
```

### 单数据实现

```rust
impl<T> Heap<T> for MyHeap<T> {
    fn push(&mut self, value: T) {
        self.vec.push(value)
    }

    fn peek(&self) -> Option<&T> {
        self.vec.first()
    }

    fn pop(&mut self) -> Option<T> {
        Some(self.vec.swap_remove(0))
    }
}
```

### 使用示例

```rust
fn main() {
    let mut heap = MyHeap::default();
    heap.push(44);
    println!("{heap:?}");
}
```

## 总结

定义出了二叉堆的主体结构，支持插入和获取一个元素，接下来支持多元素。

## 附录

### 源码

```rust
fn main() {
    let mut heap = MyHeap::default();
    heap.push(44);
    println!("{heap:?}");
    println!("{:?}", heap.peek());
    println!("{:?}", heap.pop());
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

impl<T> Heap<T> for MyHeap<T> {
    fn push(&mut self, value: T) {
        self.vec.push(value)
    }

    fn peek(&self) -> Option<&T> {
        self.vec.first()
    }

    fn pop(&mut self) -> Option<T> {
        Some(self.vec.swap_remove(0))
    }
}
```
