# 【Rust】二叉堆-标准库

## 环境

- Time 2022-04-15
- Rust 1.60.0

## 前言

### 说明

基于标准库来学习各种数据结构，并不是从头实现数据结构，未考虑实现性能。

### 特点

二叉堆的极值在最前面，可能是最大值或者最小值，又叫大顶堆或者小顶堆。

## 示例

### new

```rust
fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(44);
}
```

### push

```rust
fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(44);
}
```

### with_capacity

```rust
fn main() {
    let mut heap = BinaryHeap::with_capacity(1);
    heap.push(44);
    println!("{heap:?}");
}
```

### peek_mut

```rust
fn main() {
    let mut heap = BinaryHeap::with_capacity(10);
    (0..10).for_each(|e| heap.push(e));
    println!("{heap:?}");
    {
        let mut max = heap.peek_mut().unwrap();
        *max = -10;
    }
    println!("{heap:?}");
}
```

### pop

```rust
fn main() {
    let mut heap = BinaryHeap::with_capacity(10);
    (0..10).for_each(|e| heap.push(e));
    println!("{:?}", heap.pop());
    println!("{heap:?}");
}
```

### into_sorted_vec

```rust
fn main() {
    let mut heap = BinaryHeap::with_capacity(10);
    (0..10).for_each(|e| heap.push(e));
    println!("{:?}", heap.into_sorted_vec());
}
```

### into_vec

```rust
fn main() {
    let mut heap = BinaryHeap::with_capacity(10);
    (0..10).for_each(|e| heap.push(e));
    println!("{:?}", heap.into_vec());
}
```

### from

```rust
fn main() {
    let mut heap = BinaryHeap::with_capacity(10);
    (0..10).for_each(|e| heap.push(e));
    heap.append(&mut BinaryHeap::from([3, 6, 9]));
    println!("{:?}", heap);
}
```

### append

```rust
fn main() {
    let mut heap = BinaryHeap::with_capacity(10);
    (0..10).for_each(|e| heap.push(e));
    heap.append(&mut BinaryHeap::from([3, 6, 9]));
    println!("{:?}", heap);
}
```

### iter

```rust
fn main() {
    let mut heap = BinaryHeap::with_capacity(10);
    (0..10).for_each(|e| heap.push(e));
    heap.iter().for_each(|e| println!("{e:?}"));
}
```

### len

```rust
fn main() {
    let mut heap = BinaryHeap::with_capacity(10);
    (0..10).for_each(|e| heap.push(e));
    println!("{:?}", heap.len());
}
```

### is_empty

```rust
fn main() {
    let mut heap = BinaryHeap::with_capacity(10);
    (0..10).for_each(|e| heap.push(e));
    println!("{:?}", heap.is_empty());
}
```

## 总结

练习使用了标准库中的二叉堆。

## 附录
