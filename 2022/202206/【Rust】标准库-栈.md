# 【Rust】标准库-栈

## 环境

- Time 2022-04-02
- Rust 1.59.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构。

### 特点

栈是一种后进先出（LIFO）的数据结构。

## 示例

### 抽象数据接口

```rust
trait Stack<T> {
    /// 新建一个空栈
    fn new() -> Self;
    /// 栈的大小
    fn size(&self) -> usize;
    /// 是否为空
    fn empty(&self) -> bool;
    /// 元素入栈
    fn push(&mut self, e: T);
    /// 元素出栈
    fn pop(&mut self) -> Option<T>;
    /// 查看栈顶元素
    fn top(&self) -> Option<&T>;
}
```

### Vec实现

使用标准库 Vec 来实现。

```rust
struct MyStack<T> {
    vec: Vec<T>,
}

impl<T> Stack<T> for MyStack<T> {
    fn new() -> Self {
        MyStack { vec: Vec::new() }
    }

    fn size(&self) -> usize {
        self.vec.len()
    }

    fn empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn push(&mut self, e: T) {
        self.vec.push(e)
    }

    fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    fn top(&self) -> Option<&T> {
        self.vec.last()
    }
}
```

### 使用

```rust
fn main() {
    let mut stack = MyStack::new();
    for i in 0..10 {
        stack.push(i);
    }
    while let Some(v) = stack.pop() {
        println!("{v}");
    }
}
```

## 总结

标准库中的 Vec 内置支持栈结构。

## 附录
