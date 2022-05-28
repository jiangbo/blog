# 0037-Bytes-bytes源码阅读

## 环境

- Time 2022-05-28
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：<https://github.com/tokio-rs/bytes>

### 目标

实现 `bytes.rs` 中的一部分方法。

## Deref

通过实现 `Deref` 来实现自动解引用，解引用成字节切片。

```rust
impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}
```

## AsRef

通过实现 `AsRef`，实现了引用到引用的转换。

```rust
impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}
```

## Deref 使用

```rust
fn main() {
    let bytes = Bytes::from_static("JiangBo".as_bytes());
    println!("bytes: {:?}", &bytes[0..5])
}
```

## slice_ref

```rust
pub fn slice_ref(&self, subset: &[u8]) -> Bytes {
    if subset.is_empty() {
        return Bytes::new();
    }

    let bytes_p = self.as_ptr() as usize;
    let bytes_len = self.len();

    let sub_p = subset.as_ptr() as usize;
    let sub_len = subset.len();

    // 子切片的地址大于等于父切片的地址
    assert!(
        sub_p >= bytes_p,
        "subset pointer ({:p}) is smaller than self pointer ({:p})",
        sub_p as *const u8,
        bytes_p as *const u8,
    );
    assert!(
        sub_p + sub_len <= bytes_p + bytes_len,
        "subset is out of bounds: self = ({:p}, {}), subset = ({:p}, {})",
        bytes_p as *const u8,
        bytes_len,
        sub_p as *const u8,
        sub_len,
    );
    // 起地址
    let sub_offset = sub_p - bytes_p;
    self.slice(sub_offset..(sub_offset + sub_len))
}
```

## slice_ref 使用

```rust
fn main() {
    let bytes = Bytes::from_static("JiangBo".as_bytes());
    println!("bytes: {:p}", bytes.as_ptr());
    let sub = bytes.slice_ref(&bytes[5..7]);
    println!("bytes: {:p}", sub.as_ptr())
}
```

## with_vtable

直接从给定的 `vtable` 来创建 `Bytes`。

```rust
    pub(crate) unsafe fn with_vtable(
        ptr: *const u8,
        len: usize,
        data: AtomicPtr<()>,
        vtable: &'static Vtable,
    ) -> Bytes {
        Bytes {ptr,len, data, vtable,}
    }
```

## inc_start

将指向开始的指针往后移动。

```rust
unsafe fn inc_start(&mut self, by: usize) {
    debug_assert!(self.len >= by, "internal: inc_start out of bounds");
    self.len -= by;
    self.ptr = self.ptr.offset(by as isize);
}
```

## 总结

给 `Bytes` 实现了 `Deref`，`AsRef`，`slice_ref` 等方法。

## 附录
