# 【Rust】动态数组（五）

## 环境

- Time 2022-03-17
- Rust 1.59.0

## 概念

动态数组分配在栈上，长度可以变化。

## 示例

### append

连接两个动态数组。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    let mut vec1 = vec![44, 55];
    vec.append(&mut vec1);
    println!("{vec:?}");
}
```

### clear

清空元素。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.clear();
    println!("{vec:?}");
}
```

### len

获取长度。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    println!("{}", vec.len());
}

```

### is_empty

是否为空。

```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4];
    println!("{}", vec.is_empty());
}
```

### split_off

在指定的位置分割为两个动态数组。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    let split = vec.split_off(2);
    println!("vec: {vec:?}, split: {split:?}");
}
```

### resize

用指定值填充到指定的长度。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.resize(8, 44);
    println!("{vec:?}");
}
```

### resize_with

使用闭包填充到指定的长度。

```rust
fn main() {
    let mut vec = vec![0, 1, 2, 3, 4];
    vec.resize_with(8, Default::default);
    println!("{vec:?}");
}
```

## 总结

了解了动态数组中相关的一些方法。

## 附录
