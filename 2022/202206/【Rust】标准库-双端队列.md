# 【Rust】标准库-双端队列

## 环境

- Time 2022-04-02
- Rust 1.59.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构。

### 特点

双端队列可以在队首或者队尾加入或者删除数据。

## 示例

### 抽象数据接口

```rust
trait Deque<T> {
    /// 新建一个空队列
    fn new() -> Self;
    /// 队列的大小
    fn size(&self) -> usize;
    /// 是否为空
    fn empty(&self) -> bool;
    /// 元素入队尾
    fn push_back(&mut self, e: T);
    /// 元素入队首
    fn push_front(&mut self, e: T);
    /// 队首出元素
    fn pop_front(&mut self) -> Option<T>;
    /// 队尾出元素
    fn pop_back(&mut self) -> Option<T>;
    /// 查看队首元素
    fn front(&self) -> Option<&T>;
    /// 查看队尾元素
    fn back(&self) -> Option<&T>;
}
```

### VecDeque 实现

使用标准库 VecDeque 来实现。

```rust
struct MyDeque<T> {
    vec: VecDeque<T>,
}

impl<T> Deque<T> for MyDeque<T> {
    fn new() -> Self {
        MyDeque {
            vec: VecDeque::new(),
        }
    }

    fn size(&self) -> usize {
        self.vec.len()
    }

    fn empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn push_back(&mut self, e: T) {
        self.vec.push_back(e)
    }

    fn push_front(&mut self, e: T) {
        self.vec.push_front(e)
    }

    fn pop_front(&mut self) -> Option<T> {
        self.vec.pop_front()
    }

    fn pop_back(&mut self) -> Option<T> {
        self.vec.pop_back()
    }

    fn front(&self) -> Option<&T> {
        self.vec.front()
    }

    fn back(&self) -> Option<&T> {
        self.vec.back()
    }
}
```

### 使用

```rust
fn main() {
    let mut queue = MyDeque::new();
    for i in 0..10 {
        queue.push_back(i);
    }
    while let Some(v) = queue.pop_back() {
        println!("{v}");
    }
}
```

## 总结

标准库中的 VecDeque 内置支持双端队列结构，同时也可以支持队列和栈结构。

## 附录
