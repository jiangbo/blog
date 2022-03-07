# 【Rust】数组切片（五）

## 环境

- Time 2022-03-07
- Rust 1.59.0

## 概念

数组切片是引用数组中连续的一部分。

## 示例

### contains

是否包含某个元素。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let contains = arr.contains(&2);
    println!("{contains}");
}
```

### fill

用某个值或者某个方法进行填充。

```rust
fn main() {
    let mut arr = [0, 1, 2, 3, 4];
    arr.fill(4);
    println!("{arr:?}");
    arr.fill_with(Default::default);
    println!("{arr:?}");
}
```

### first

获取第一个元素，可变版本 `first_mut`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let first = arr.first();
    println!("{first:?}");
}
```

### is_empty

是否为空。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let empty = arr.is_empty();
    println!("{empty:?}");
}
```

### last

获取最后一个元素，可变版本 `last_mut`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let last = arr.last();
    println!("{last:?}");
}
```

### len

获取长度。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let len = arr.len();
    println!("{len:?}");
}
```

## 总结

了解了数组切片中相关的一些方法。

## 附录
