# 【Rust】数组切片（三）

## 环境

- Time 2022-03-01
- Rust 1.59.0

## 概念

数组切片是引用数组中连续的一部分。

## 示例

### clone

直接克隆数据。

```rust
fn main() {
    let arr = ["Python", "Java", "C++", "Rust"];
    let src = arr.map(|e| e.to_string());
    let dst = src.clone();
    drop(src);
    println!("{dst:?}");
}
```

### clone_from

从数组引用进行克隆。

```rust
fn main() {
    let mut dst: [String; 4] = Default::default();
    let arr = ["Python", "Java", "C++", "Rust"];
    let src = arr.map(|e| e.to_string());
    dst.clone_from(&src);
    println!("{dst:?}");
}
```

### clone_from_slice

从切片克隆。

```rust
fn main() {
    let mut dst: [String; 2] = Default::default();
    let arr = ["Python", "Java", "C++", "Rust"];
    let src = arr.map(|e| e.to_string());
    dst.clone_from_slice(&src[2..]); // 长度不对会报错
    println!("{dst:?}");
}
```

### copy_from_slice

从切片复制。

```rust
fn main() {
    let mut dst = [0, 0];
    let src = [0, 1, 2, 3, 4];
    dst.copy_from_slice(&src[3..]);
    println!("{dst:?}");
}
```

### copy_within

将自身的一部分覆盖到另一部分。

```rust
fn main() {
    let mut src = [0, 1, 2, 3, 4];
    src.copy_within(..2, 3);
    println!("{src:?}");
}
```

## 总结

了解了数组切片中克隆和复制的相关方法。

## 附录
