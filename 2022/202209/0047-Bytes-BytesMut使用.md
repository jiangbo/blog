# 0047-BytesMut使用

## 环境

- Time 2022-05-29
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：<https://docs.rs/bytes/latest/bytes/struct.BytesMut.html>

### 目标

使用 `BytesMut` 的方法。

## with_capacity

```rust
fn main() {
    let bytes = BytesMut::with_capacity(44);
    println!("{:?}",bytes);
}
```

## new

```rust
fn main() {
    let bytes = BytesMut::new();
    println!("{:?}", bytes);
}
```

## capacity

```rust
fn main() {
    let bytes = BytesMut::with_capacity(44);
    println!("{:?}", bytes.capacity());
}
```

## len

```rust
fn main() {
    let bytes = BytesMut::from("JiangBo");
    println!("{:?}", bytes.len());
}
```

## put

```rust
fn main() {
    let mut bytes = BytesMut::from("JiangBo");
    bytes.put(b"!".as_ref());
    println!("{:?}", bytes);
}
```

## freeze

```rust
fn main() {
    let bytes = BytesMut::from("JiangBo");
    println!("{:?}", bytes.freeze());
}
```

## split_off

```rust
fn main() {
    let mut bytes = BytesMut::from("JiangBo");
    println!("{:?}", bytes.split_off(5));
}
```

## split_to

```rust
fn main() {
    let mut bytes = BytesMut::from("JiangBo");
    println!("{:?}", bytes.split_to(5));
}
```

## split

```rust
fn main() {
    let mut bytes = BytesMut::from("JiangBo");
    println!("{:?}", bytes.split());
}
```

## extend_from_slice

```rust
fn main() {
    let mut bytes = BytesMut::from("Hello ");
    bytes.extend_from_slice("JiangBo".as_ref());
    println!("{:?}", bytes);
}
```

## reverse

```rust
fn main() {
    let mut bytes = BytesMut::from("JiangBo");
    bytes.reverse();
    println!("{:?}", bytes);
}
```

## clear

```rust
fn main() {
    let mut bytes = BytesMut::from("JiangBo");
    bytes.clear();
    println!("{:?}", bytes);
}
```

## resize

```rust
fn main() {
    let mut bytes = BytesMut::from("JiangBo");
    bytes.resize(10, b'o');
    println!("{:?}", bytes);
}
```

## 总结

使用了 `BytesMut` 中定义的一些方法。

## 附录
