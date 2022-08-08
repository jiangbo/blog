# 0048-Bytes-BufMut使用

## 环境

- Time 2022-05-29
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：<https://docs.rs/bytes/latest/bytes/trait.BufMut.html>

### 目标

使用 `BufMut` 的方法。

## remaining_mut

```rust
fn main() {
    let mut dst = [0; 10];
    let mut buf = &mut dst[..];
    println!("{:?}", buf.remaining_mut());
    buf.put(&b"hello"[..]);
    println!("{:?}", buf.remaining_mut());
}
```

## new

```rust
fn main() {
    let mut buf = Vec::with_capacity(16);
    buf.chunk_mut()[0..2].copy_from_slice(b"he");
    unsafe { buf.advance_mut(2) };
    buf.chunk_mut()[0..3].copy_from_slice(b"llo");

    unsafe {
        buf.advance_mut(3);
    }

    assert_eq!(5, buf.len());
    assert_eq!(buf, b"hello");
}
```

## chunk_mut

```rust
fn main() {
    let mut buf = Vec::with_capacity(16);
    unsafe {
        buf.chunk_mut()[0..].as_mut_ptr().write(b'h');
        buf.chunk_mut()[1..].as_mut_ptr().write(b'e');
        buf.advance_mut(2);
        buf.chunk_mut()[0..].as_mut_ptr().write(b'l');
        buf.chunk_mut()[1..].as_mut_ptr().write(b'l');
        buf.chunk_mut()[2..].as_mut_ptr().write(b'o');

        buf.advance_mut(3);
    }

    assert_eq!(5, buf.len());
    assert_eq!(buf, b"hello");
}
```

## has_remaining_mut

```rust
fn main() {
    let mut dst = [0; 5];
    let buf = &mut dst[..];
    println!("{:?}", buf.has_remaining_mut());
}
```

## put

```rust
fn main() {
    let mut buf = vec![];

    buf.put_u8(b'h');
    buf.put(&b"ello"[..]);
    buf.put(b" world".as_ref());
    assert_eq!(buf, b"hello world");
}
```

## put_slice

```rust
fn main() {
    let mut dst = [0; 6];
    let mut buf = &mut dst[..];
    buf.put_slice(b"hello");
    assert_eq!(1, buf.remaining_mut());
    assert_eq!(b"hello\0", &dst);
}
```

## put_bytes

```rust
fn main() {
    let mut dst = [0; 6];
    let mut buf = &mut dst[..];
    buf.put_bytes(b'a', 4);
    assert_eq!(2, buf.remaining_mut());
    assert_eq!(b"aaaa\0\0", &dst);
}
```

## put_u8

```rust
fn main() {
    let mut buf = vec![];
    buf.put_u8(0x01);
    assert_eq!(buf, b"\x01");
}
```

## 总结

使用了 `BufMut` 中定义的一些方法。

## 附录
