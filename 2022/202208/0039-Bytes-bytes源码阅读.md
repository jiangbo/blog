# 0039-Bytes-bytes源码阅读

## 环境

- Time 2022-05-28
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：<https://github.com/tokio-rs/bytes>

### 目标

实现 `bytes.rs` 中的一部分方法。

## 线程安全

实现了两个线程安全的标记接口。

```rust
unsafe impl Send for Bytes {}
unsafe impl Sync for Bytes {}
```

## Hash

实现了 `Hash` 函数。

```rust
impl hash::Hash for Bytes {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.as_slice().hash(state);
    }
}
```

## Borrow

```rust
impl Borrow<[u8]> for Bytes {
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}
```

## PartialEq

`Bytes` 实现了很多类型的比较方法，主要是方便对不同类型直接进行比较，下面只列出自己和自己的比较。

```rust
impl PartialEq for Bytes {
    fn eq(&self, other: &Bytes) -> bool {
        self.as_slice() == other.as_slice()
    }
}
```

## PartialOrd

```rust
impl PartialOrd for Bytes {
    fn partial_cmp(&self, other: &Bytes) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}
```

## Ord

```rust
impl Ord for Bytes {
    fn cmp(&self, other: &Bytes) -> cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl Eq for Bytes {}
```

## Vtable

`Vtable` 实现 `Debug`。

```rust
impl fmt::Debug for Vtable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vtable")
            .field("clone", &(self.clone as *const ()))
            .field("drop", &(self.drop as *const ()))
            .finish()
    }
}
```

## 总结

给 `Bytes` 实现了 `Hash`，`Borrow`，`Eq` 等方法。

## 附录
