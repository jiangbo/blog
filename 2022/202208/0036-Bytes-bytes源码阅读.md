# 0036-Bytes-bytes源码阅读

## 环境

- Time 2022-05-28
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：<https://github.com/tokio-rs/bytes>

### 目标

实现 `bytes.rs` 中的一部分方法。

## Drop

通过自定义的 `Vtable` 来实现 `drop` 方法。

```rust
impl Drop for Bytes {
    fn drop(&mut self) {
        unsafe { (self.vtable.drop)(&mut self.data, self.ptr, self.len) }
    }
}
```

## Clone

通过自定义的 `Vtable` 来实现 `clone` 方法。

```rust
impl Clone for Bytes {
    fn clone(&self) -> Bytes {
        unsafe { (self.vtable.clone)(&self.data, self.ptr, self.len) }
    }
}
```

## RangeBounds

`RangeBounds` 是 Rust 中的一种范围语法，有六种情况。

```rust
pub enum Bound<T> {
    Included(T),
    Excluded(T),
    Unbounded,
}

pub trait RangeBounds<T: ?Sized> {
    fn start_bound(&self) -> Bound<&T>;
    fn end_bound(&self) -> Bound<&T>;
}
```

## range

定义一个范围方法，可以根据给定的范围，拿到起始和结束的索引。

```rust
fn range(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
    let len = self.len();

    let begin = match range.start_bound() {
        Bound::Included(&n) => n,
        Bound::Excluded(&n) => n + 1,
        Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
        Bound::Included(&n) => n.checked_add(1).expect("out of range"),
        Bound::Excluded(&n) => n,
        Bound::Unbounded => len,
    };

    assert!(
        begin <= end,
        "range start must not be greater than end: {begin:?} <= {end:?}"
    );
    assert!(end <= len, "range end out of bounds: {end:?} <= {len:?}");
    (begin, end)
}
```

## slice

定义一个切片方法，可以支持索引语法。

```rust
pub fn slice(&self, range: impl RangeBounds<usize>) -> Bytes {
    let (begin, end) = self.range(range);
    if end == begin {
        return Bytes::new();
    }

    let mut ret = self.clone();
    ret.len = end - begin;
    ret.ptr = unsafe { ret.ptr.offset(begin as isize) };
    ret
}
```

## slice 使用

`slice` 方法可以从给定的范围中，获取一个 `Bytes`。

```rust
fn main() {
    let bytes = Bytes::from_static("JiangBo".as_bytes());
    println!("length: {:?}", bytes.slice(0..5).len())
}
```

## 总结

给 `Bytes` 实现了 `drop`，`clone`，`slice` 方法。

## 附录
