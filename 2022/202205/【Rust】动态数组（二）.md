# 【Rust】动态数组（二）

## 环境

- Time 2022-03-16
- Rust 1.59.0

## 概念

动态数组分配在栈上，长度可以变化。

## 示例

### reserve

保留额外空间，相当于扩容，容量可能会比扩容的大，带异常版本：`try_reserve`。

```rust
fn main() {
    let mut vec = Vec::new();
    vec.push(0);
    vec.reserve(4);
    println!("{}", vec.capacity()); // 8
}
```

### reserve_exact

```rust
fn main() {
    let mut vec = Vec::new();
    vec.push(0);
    vec.reserve_exact(4);
    println!("{}", vec.capacity()); // 5
}
```

### shrink_to_fit

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.reserve(4);
    println!("{}", vec.capacity());
    vec.shrink_to_fit();
    println!("{}", vec.capacity());
}
```

### shrink_to

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.reserve(4);
    println!("{}", vec.capacity());
    vec.shrink_to(7);
    println!("{}", vec.capacity());
}
```

### into_boxed_slice

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let slice = vec.into_boxed_slice();
    println!("{slice:?}");
}
```

## 总结

了解了动态数组中相关的一些方法。

## 附录
