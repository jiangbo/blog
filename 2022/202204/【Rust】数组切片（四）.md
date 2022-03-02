# 【Rust】数组切片（四）

## 环境

- Time 2022-03-01
- Rust 1.59.0

## 概念

数组切片是引用数组中连续的一部分。

## 示例

### concat

直接将切片的每个值连接起来。

```rust
fn main() {
    let arr = [[0, 1], [2, 3], [4, 5]];
    let concat = arr.concat();
    println!("{concat:?}");
}
```

### join

用特定的值将切片的每个值连接起来。

```rust
fn main() {
    let hello = ["hello", "world"].join(" ");
    println!("{hello}");
    let arr = [[0, 1], [2, 3], [4, 5]];
    let join = arr.join(&[0, 0][..]);
    println!("{join:?}");
}
```

### ends_with

以什么结尾。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let end1 = arr.ends_with(&[4]);
    println!("{end1:?}");
    let end2 = arr.ends_with(&[3, 4]);
    println!("{end2:?}");
}
```

### eq

相等判断。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let flag = arr.eq(&[0, 0, 0, 0, 0]);
    println!("{flag:?}");
}
```

### get

根据索引获取元素，比 `[]` 操作安全，可变版本 `get_mut`，不检查版本 `get_unchecked`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let v1 = arr.get(4); // 超出索引返回 None
    println!("{v1:?}");
    let v2 = arr.get(1..4); // 获取范围
    println!("{v2:?}");
}
```

## 总结

了解了数组切片中相关的一些方法。

## 附录
