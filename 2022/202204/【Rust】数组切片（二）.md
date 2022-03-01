# 【Rust】数组切片（二）

## 环境

- Time 2022-03-01
- Rust 1.59.0

## 概念

数组切片是引用数组中连续的一部分。

## 示例

### chunks

按照固定的长度，截取成一个迭代器，最后一个可能不满足长度要求。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    let chunks = arr.chunks(2);
    for ele in chunks {
        println!("{ele:?}");
    }
}
```

### chunks_mut

按照固定的长度，截取成一个迭代器，其中的元素可变，最后一个可能不满足长度要求。

> Rust 中的方法如果以 `mut` 结尾，表示返回的是一个可变的变量。

```rust
fn main() {
    let mut arr = [0, 1, 2, 3, 4];
    for chunk in arr.chunks_mut(2) {
        for ele in chunk {
            *ele += 1
        }
    }
    println!("{arr:?}");
}
```

### chunks_exact

精确截取，如果最后一个元素长度不符合要求，则不会输出，可变版本 `chunks_exact_mut`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    for chunk in arr.chunks_exact(2) {
        println!("{chunk:?}");
    }
}
```

### rchunks

从右往左截取，元素的顺序不变，可变版本 `rchunks_mut`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    for chunk in arr.rchunks(2) {
        println!("{chunk:?}");
    }
}
```

### rchunks_exact

从右往左精确截取，元素的顺序不变，可变版本 `rchunks_exact_mut`。

```rust
fn main() {
    let arr = [0, 1, 2, 3, 4];
    for chunk in arr.rchunks_exact(2) {
        println!("{chunk:?}");
    }
}
```

## 总结

了解了数组切片中 `chunk` 的几种方式，包括普通截取，精确截取，反向截取。

## 附录
