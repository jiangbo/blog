# 【Rust】标准库-队列

## 环境

- Time 2022-04-02
- Rust 1.59.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构。

### 特点

队列是一种先进先出（FIFO）的数据结构。

## 示例

### 抽象数据接口

```rust
trait Queue<T> {
    /// 新建一个空队列
    fn new() -> Self;
    /// 队列的大小
    fn size(&self) -> usize;
    /// 是否为空
    fn empty(&self) -> bool;
    /// 元素入队
    fn enqueue(&mut self, e: T);
    /// 元素出队
    fn dequeue(&mut self) -> Option<T>;
    /// 查看队首元素
    fn front(&self) -> Option<&T>;
}
```

### Vec实现

使用标准库 Vec 来实现。

```rust
struct MyQueue<T> {
    vec: Vec<T>,
}

impl<T> Queue<T> for MyQueue<T> {
    fn new() -> Self {
        MyQueue { vec: Vec::new() }
    }

    fn size(&self) -> usize {
        self.vec.len()
    }

    fn empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn enqueue(&mut self, e: T) {
        self.vec.push(e)
    }

    fn dequeue(&mut self) -> Option<T> {
        match self.empty() {
            true => None,
            false => Some(self.vec.remove(0)),
        }
    }

    fn front(&self) -> Option<&T> {
        self.vec.first()
    }
}
```

### VecDeque 实现

使用默认的 Vec 来实现，在入队或者出队时消耗很大，所以标准直接提供了 `VecDeque`。  
下面使用标准库提供的这个结构来实现。

```rust
struct MyQueue<T> {
    vec: VecDeque<T>,
}

impl<T> Queue<T> for MyQueue<T> {
    fn new() -> Self {
        MyQueue {
            vec: VecDeque::new(),
        }
    }

    fn size(&self) -> usize {
        self.vec.len()
    }

    fn empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn enqueue(&mut self, e: T) {
        self.vec.push_back(e)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.vec.pop_front()
    }

    fn front(&self) -> Option<&T> {
        self.vec.front()
    }
}

```

### 使用

```rust
fn main() {
    let mut queue = MyQueue::new();
    for i in 0..10 {
        queue.enqueue(i);
    }
    while let Some(v) = queue.dequeue() {
        println!("{v}");
    }
}
```

## 总结

标准库中的 VecDeque 内置支持队列结构。

## 附录
