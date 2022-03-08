# 【Rust】数组切片（七）

## 环境

- Time 2022-03-08
- Rust 1.59.0

## 概念

数组切片是引用数组中连续的一部分。

## 示例

### split_first

截取第一个元素，可变版本 `split_first_mut`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    if let Some((first, right)) = arr.split_first() {
        println!("first: {first:?}, right: {right:?}")
    }
}
```

### split_inclusive

分割，包含满足条件的元素，可变版本 `split_inclusive_mut`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    arr.split_inclusive(|n| n == &2)
        .for_each(|e| println!("{e:?}"));
}
```

### split_last

截取最后一个元素，可变版本 `split_last_mut`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    if let Some((last, right)) = arr.split_last() {
        println!("last: {last:?}, right: {right:?}")
    }
}
```

### strip_prefix

切割前缀，如果没有匹配前缀返回 None，否则返回 Some。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    if let Some(strip) = arr.strip_prefix(&[0]) {
        println!("strip: {strip:?}")
    }
}
```

### strip_suffix

切割后缀，如果没有匹配后缀返回 None，否则返回 Some。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    if let Some(strip) = arr.strip_suffix(&[4]) {
        println!("strip: {strip:?}")
    }
}
```

### starts_with

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    if arr.starts_with(&[0, 1]) {
        println!("start with: true")
    }
}
```

### swap

根据位置交换切片中两个值的位置。

```rust
fn main() {
    let mut arr = [0, 1, 2, 3, 4];
    arr.swap(1, 4);
    println!("{:?}", arr);
}
```

### to_vec

将切片转为动态数组。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let vec = arr.to_vec();
    println!("{:?}", vec);
}
```

### windows

窗口函数，可以每次取固定个数的值。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let vec = arr.windows(2);
    for ele in vec {
        println!("{ele:?}");
    }
}
```

## 总结

了解了数组切片中相关的一些方法。

## 附录
