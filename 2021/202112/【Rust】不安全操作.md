# 【Rust】不安全操作

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/unsafe.html>  

## 示例

### 原始指针

```rust
fn main() {
    let raw_p: *const u32 = &10;
    unsafe {
        assert!(*raw_p == 10);
    }
}
```

### 不安全函数

```rust
[package]
use std::slice;

fn main() {
    let some_vector = vec![1, 2, 3, 4];
    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);
        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
```

## 总结

了解了 Rust 中不安全操作的使用方式，需要使用 `unsafe` 关键字包裹。

## 附录
