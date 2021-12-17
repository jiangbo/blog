# 【Rust】原始标识符

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/compatibility/raw_identifiers.html>  

## 示例

原始标识符可以避免由于新增加关键字导致的不兼容问题，使用 `r#` 来使用。

### main.rs

```rust
extern crate foo;

fn main() {
    foo::r#try();
}
```

## 总结

了解了 Rust 中原始标识符的使用方式和作用。

## 附录
