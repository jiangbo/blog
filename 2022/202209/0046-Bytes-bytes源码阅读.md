# 0046-Bytes-bytes源码阅读

## 环境

- Time 2022-05-29
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：<https://docs.rs/bytes/latest/bytes/trait.Buf.html>

### 目标

`Bytes` 实现了 `Buf`，使用一下其中的方法。

## remaining

```rust
fn main() {
    let mut buf = b"JiangBo".as_ref();
    println!("{:?}", buf.remaining());
    println!("{:?}", buf.get_u8() as char);
    println!("{:?}", buf.remaining());
}
```

## chunk

```rust
fn main() {
    let buf = b"JiangBo".as_ref();
    println!("{:?}", buf.chunk());
}
```

## advance

```rust
fn main() {
    let mut buf = b"JiangBo".as_ref();
    buf.advance(5);
    println!("{:?}", buf);
}
```

## copy_to_slice

```rust
fn main() {
    let mut buf = b"JiangBo".as_ref();
    let mut dst = [0; 5];
    buf.copy_to_slice(&mut dst);
    println!("{:?}", Bytes::from(buf));
}
```

## get_u8

```rust
fn main() {
    let mut buf = b"JiangBo".as_ref();
    println!("{:?}", buf.get_u8() as char);
    println!("{:?}", Bytes::from(buf));
}
```

## copy_to_bytes

```rust
fn main() {
    let mut buf = b"JiangBo".as_ref();
    let bytes = buf.copy_to_bytes(5);
    println!("{:?}", bytes);
}
```

## take

```rust
fn main() {
    let buf = b"JiangBo".as_ref();
    println!("{:?}", buf.take(5));
}
```

## chain

```rust
fn main() {
    let buf = b"Hello ".as_ref();
    let mut chain = buf.chain(b"JiangBo".as_ref());
    let bytes = chain.copy_to_bytes(13);
    println!("{:?}", bytes);
}
```

## 总结

使用了 `Buf` 中定义的一些方法。

## 附录
