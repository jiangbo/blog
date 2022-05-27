# 0034-Bytes-bytes源码阅读

## 环境

- Time 2022-05-27
- Rust 1.61.0
- Bytes 1.1.0

## 前言

### 说明

参考：<https://github.com/tokio-rs/bytes>

### 目标

了解 `bytes.rs` 中 `Bytes` 的结构定义。

## lib.rs

首先将 `bytes.rs` 引入到 `lib.rs` 中，然后进行了重新导出 `Bytes`。

```rust
mod bytes;
pub use crate::bytes::Bytes;
```

## Bytes

`Bytes` 的结构体定义，一个原始指针和原子指针，加上一个 `usize` 的长度和一个引用。占用空间 4 个 `usize` 的大小。

```rust
pub struct Bytes {
    ptr: *const u8,
    len: usize,
    data: AtomicPtr<()>,
    vtable: &'static Vtable,
}
```

## Vtable

`Vtable` 定义了两个函数指针，一个 `clone`，一个 `drop`。

```rust
pub(crate) struct Vtable {
    pub clone: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Bytes,
    pub drop: unsafe fn(&mut AtomicPtr<()>, *const u8, usize),
}
```

## AtomicPtr

`AtomicPtr` 是一个原子指针，可以在线程间安全地共享和移动。

```rust
#[cfg(target_has_atomic_load_store = "ptr")]
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T> Send for AtomicPtr<T> {}
#[cfg(target_has_atomic_load_store = "ptr")]
#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T> Sync for AtomicPtr<T> {}
```

可以看到在标准库中，为 `AtomicPtr` 实现了 `Send` 和 `Sync`。

### UnsafeCell

`AtomicPtr` 内部是一个 `UnsafeCell`，其不是线程安全的。

```rust
pub struct AtomicPtr<T> {
    p: UnsafeCell<*mut T>,
}

#[stable(feature = "rust1", since = "1.0.0")]
unsafe impl<T: ?Sized> Send for RefCell<T> where T: Send {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Sync for RefCell<T> {}
```

`UnsafeCell` 未实现 `Sync`，并且只对实现了 `Send` 的 T 才实现 `Send`。不过原始指针未实现 `Sync` 和 `Send`，所以 `UnsafeCell<*mut T` 相当于未实现 `Sync` 和 `Send`。

## 原始指针

可以看到原始指针既没有实现 `Sync`，也没有实现 `Send`。

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Sync for *const T {}
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Sync for *mut T {}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Send for *const T {}
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> !Send for *mut T {}
```

## 总结

查看了 `Bytes` 结构体中，每个字段的定义，以及 `Vtable` 的定义。

## 附录
