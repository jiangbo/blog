# 0044-Bytes-bytes源码阅读

## 环境

- Time 2022-05-29
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：<https://github.com/tokio-rs/bytes>

### 目标

`Buf` 是一个 trait，里面有几个方法需要实现，`Bytes` 实现了 `Buf`。

## remaining

```rust
fn remaining(&self) -> usize {
    self.len()
}
```

## chunk

```rust
fn chunk(&self) -> &[u8] {
    self.as_slice()
}
```

## advance

```rust
fn advance(&mut self, cnt: usize) {
    assert!(
        cnt <= self.len(),
        "cannot advance past `remaining`: {:?} <= {:?}",
        cnt,
        self.len(),
    );

    unsafe {
        self.inc_start(cnt);
    }
}
```

## copy_to_bytes

```rust
fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
    if len == self.remaining() {
        core::mem::replace(self, Bytes::new())
    } else {
        let ret = self.slice(..len);
        self.advance(len);
        ret
    }
}
```

## truncate

还有之前没有实现的几个方法。

```rust
pub fn truncate(&mut self, len: usize) {
    if len < self.len {
        if self.vtable as *const Vtable == &PROMOTABLE_EVEN_VTABLE
            || self.vtable as *const Vtable == &PROMOTABLE_ODD_VTABLE
        {
            drop(self.split_off(len));
        } else {
            self.len = len;
        }
    }
}
```

## clear

```rust
pub fn clear(&mut self) {
    self.truncate(0);
}
```

## 总结

对 `Bytes` 实现了 `Buf`，补上了之前未实现的方法。

## 附录
