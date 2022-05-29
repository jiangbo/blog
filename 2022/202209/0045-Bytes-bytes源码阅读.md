# 0045-Bytes-bytes源码阅读

## 环境

- Time 2022-05-29
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：<https://github.com/tokio-rs/bytes>

### 目标

`Bytes` 的源码基本上看完了，简单使用一下其中的方法。

## new

```rust
fn main() {
    let bytes = Bytes::new();
    println!("{:?}", bytes);
}
```

## from_static

```rust
fn main() {
    let bytes = Bytes::from_static(b"JiangBo");
    println!("{:?}", bytes);
}
```

## len

```rust
fn main() {
    let bytes = Bytes::from_static(b"JiangBo");
    println!("{:?}", bytes.len());
}
```

## copy_from_slice

```rust
fn main() {
    let bytes = Bytes::copy_from_slice(&b"JiangBo"[..5]);
    println!("{:?}", bytes);
}
```

## slice_ref

```rust
fn main() {
    let bytes = Bytes::from_static(b"JiangBo");
    let slice = &bytes.as_ref()[..5];
    let sub = bytes.slice_ref(slice);
    println!("{:?}", sub);
}
```

## split_off

```rust
fn main() {
    let mut b1 = Bytes::from_static(b"JiangBo");
    let b2 = b1.split_off(5);
    println!("{:?}", b1);
    println!("{:?}", b2);
}
```

## split_to

```rust
fn main() {
    let mut b1 = Bytes::from_static(b"JiangBo");
    let b2 = b1.split_to(5);
    println!("{:?}", b1);
    println!("{:?}", b2);
}
```

## truncate

```rust
fn main() {
    let mut bytes = Bytes::from_static(b"JiangBo");
    bytes.truncate(5);
    println!("{:?}", bytes);
}
```

## clear

```rust
fn main() {
    let mut bytes = Bytes::from_static(b"JiangBo");
    bytes.clear();
    println!("{:?}", bytes);
}
```

## 总结

使用了 `Bytes` 中定义的一些方法。

## 附录
