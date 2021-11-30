# 【Rust】开发依赖

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/testing/dev_dependencies.html>  

## 示例

### main.rs

```rust
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}

fn main(){
    
}
```

### Cargo.toml

```toml
[package]
name = "rust"
version = "0.1.0"
edition = "2021"

[dependencies]

[dev-dependencies]
pretty_assertions = "0.4.0"
```

## 总结

了解了 Rust 中开发依赖的使用方式。

## 附录
