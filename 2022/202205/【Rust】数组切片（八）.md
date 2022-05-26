# 【Rust】数组切片（八）

## 环境

- Time 2022-03-08
- Rust 1.59.0

## 概念

数组切片是引用数组中连续的一部分。

## 示例

### sort

对数组切片进行排序。

```rust
fn main() {
    let mut arr = [0, 4, 3, 1, 2];
    arr.sort();
    println!("{arr:?}");
}
```

### sort_unstable

不稳定排序，一般来说比稳定排序快。

```rust
fn main() {
    let mut arr = [0, 4, 3, 1, 2];
    arr.sort_unstable();
    println!("{arr:?}");
}
```

### sort_unstable_by

自定义排序规则。

```rust
fn main() {
    let mut arr = [0, 4, 3, 1, 2];
    arr.sort_unstable_by(|a, b| b.cmp(a));
    println!("{arr:?}");
}
```

### sort_unstable_by_key

```rust
fn main() {
    let mut arr = [0, 4, 3, 1, 2];
    arr.sort_unstable_by_key(|e| e % 2 == 0);
    println!("{arr:?}");
}
```

### select_nth_unstable

```rust
fn main() {
    let mut arr = [-5i32, 4, 1, -3, 2];
    arr.select_nth_unstable(2);
    println!("{arr:?}");
}
```

## 总结

了解了数组切片中相关的一些方法。

## 附录
