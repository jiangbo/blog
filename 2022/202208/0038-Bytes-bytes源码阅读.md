# 0038-Bytes-bytes源码阅读

## 环境

- Time 2022-05-28
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：<https://github.com/tokio-rs/bytes>

### 目标

实现 `bytes.rs` 中的一部分方法。

## split_off

在中间进行切割，分成两半。

```rust
pub fn split_off(&mut self, at: usize) -> Bytes {
    // 分割的长度必须小于等于Bytes的长度
    assert!(
        at <= self.len(),
        "split_off out of bounds: {:?} <= {:?}",
        at,
        self.len(),
    );

    // 如果从最后一位切割，相当于被切的不变，返回一个空的Bytes
    if at == self.len() {
        return Bytes::new();
    }

    // 如果从头开始切换，相当于被切的变成了空，返回被切的Bytes
    if at == 0 {
        return mem::replace(self, Bytes::new());
    }

    // 复制不会底层字节，只会复制四个usize的长度
    let mut ret = self.clone();
    // 被切割了，所以长度变短了
    self.len = at;
    // 把指向开头的指针移动到被切割的位置，并且变更长度
    unsafe { ret.inc_start(at) };
    ret
}
```

## split_to

`split_off` 是把后面的切割下来，前面的保留，而 `split_to` 相反。

```rust
pub fn split_to(&mut self, at: usize) -> Bytes {
    assert!(
        at <= self.len(),
        "split_to out of bounds: {:?} <= {:?}",
        at,
        self.len(),
    );

    if at == self.len() {
        return mem::replace(self, Bytes::new());
    }

    if at == 0 {
        return Bytes::new();
    }

    let mut ret = self.clone();
    unsafe { self.inc_start(at) };
    ret.len = at;
    ret
}
```

## Default

```rust
impl Default for Bytes {
    fn default() -> Bytes {
        Bytes::new()
    }
}
```

## Default 使用

```rust
fn main() {
    let _ = Bytes::default();
}
```

## from

```rust
impl From<&'static [u8]> for Bytes {
    fn from(slice: &'static [u8]) -> Bytes {
        Bytes::from_static(slice)
    }
}

impl From<&'static str> for Bytes {
    fn from(slice: &'static str) -> Bytes {
        Bytes::from_static(slice.as_bytes())
    }
}
```

## from 使用

```rust
fn main() {
    let _: Bytes = "Hello, world!".into();
    let _: Bytes = b"Hello, world!".as_slice().into();
}
```

## 总结

给 `Bytes` 实现了 `split_off`，`split_to`，`from` 等方法。

## 附录
