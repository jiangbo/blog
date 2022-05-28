# 0035-Bytes-bytes源码阅读

## 环境

- Time 2022-05-28
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：<https://github.com/tokio-rs/bytes>

### 目标

了解从静态生命周期的字节中创建 `bytes.rs`，以及实现一部分方法。

## from_static

根据一个静态生命周期的字节切片，来生成 `Bytes`。

```rust
pub const fn from_static(bytes: &'static [u8]) -> Bytes {
    Bytes {
        ptr: bytes.as_ptr(),
        len: bytes.len(),
        data: AtomicPtr::new(ptr::null_mut()),
        vtable: &STATIC_VTABLE,
    }
}
```

## STATIC_VTABLE

静态的 `Vtable`，clone 的时候，直接从给定的指针和长度生成一个 `Bytes`，drop 的时候，什么都不需要做。

```rust
const STATIC_VTABLE: Vtable = Vtable {
    clone: static_clone,
    drop: static_drop,
};

unsafe fn static_clone(_: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Bytes {
    let slice = slice::from_raw_parts(ptr, len);
    Bytes::from_static(slice)
}

unsafe fn static_drop(_: &mut AtomicPtr<()>, _: *const u8, _: usize) {}
```

## from_static 使用

忽略其中的未使用的警告，可以正常运行。

```rust
fn main() {
    let name = "JiangBo".as_bytes();
    let _ = Bytes::from_static(name);
}
```

## new

一个空的字节切片来生成 `Bytes`。

```rust
pub const fn new() -> Bytes {
    const EMPTY: &[u8] = &[];
    Bytes::from_static(EMPTY)
}
```

### new 使用

```rust
fn main() {
    let _ = Bytes::new();
}
```

## len

获取已存在字节的长度。

```rust
pub fn len(&self) -> usize {
    self.len
}
```

## len 使用

```rust
fn main() {
    let mut bytes = Bytes::new();
    println!("empty length: {}", bytes.len());

    bytes = Bytes::from_static("JiangBo".as_bytes());
    println!("length: {}", bytes.len())
}
```

## is_empty

```rust
pub fn is_empty(&self) -> bool {
    self.len == 0
}
```

## as_slice

将其中报错的字节序列转成字节切片返回。

```rust
fn as_slice(&self) -> &[u8] {
    unsafe { slice::from_raw_parts(self.ptr, self.len) }
}
```

## 总结

给 `Bytes` 实现了 `new`，`from_static`，`len`，`as_slice` 等方法。

## 附录
