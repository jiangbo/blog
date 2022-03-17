# 【Rust】动态数组（六）

## 环境

- Time 2022-03-17
- Rust 1.59.0

## 概念

动态数组分配在栈上，长度可以变化。

## 示例

### leak

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    let leak = vec.leak();
    println!("{leak:?}");
}
```

### extend

拼接一个迭代器。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.extend([44, 55]);
    println!("{vec:?}");
}
```

### extend_from_slice

拼接一个切片。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.extend_from_slice(&[44, 55]);
    println!("{vec:?}");
}
```

### extend_from_within

拼接自身的一部分。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.extend_from_within(2..);
    println!("{vec:?}");
}
```

### splice

将一部分内容替换。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.splice(2..=3, [44, 55]);
    println!("{vec:?}");
}
```

### Deref

动态数组可以自动解引用成切片，然后使用切片的方法。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    println!("{:?}", vec.first());
}
```

## 总结

了解了动态数组中相关的一些方法。

## 附录
