# 【Rust】依赖管理

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/cargo/deps.html>  

## 示例

rust 的依赖管理可以使用 cargo，只需要在 Cargo.toml 中申明需要的依赖就行。
默认情况下，cargo 从 crates.io 去下载依赖。

### crates.io

```toml
[package]
name = "rust"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = "2.27.1"
```

### git 仓库

```toml
[package]
name = "rust"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand" }
```

### 本地路径

如果是相对路径，以 Cargo.toml 所在目录为准。

```toml
[package]
name = "rust"
version = "0.1.0"
edition = "2021"

[dependencies]
bar = { path = "../bar" }
```

## 总结

了解了 Rust 中依赖管理的工具 cargo，以及怎么添加依赖。

## 附录
