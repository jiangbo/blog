# 【Rust】集成测试

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/testing/integration_testing.html>  

## 示例

### Cargo.toml

```toml
[package]
name = "adder"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### lib.rs

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### integration_test.rs

```rust
#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
```

## 总结

了解了 Rust 中集成测试的使用方法。

## 附录
