# 【Rust】可执行文件

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/cargo/conventions.html>  

## 示例

默认情况下，生成的可执行文件和 cargo 中定义的一样，下面的方式可以生成多个可执行文件。

### 项目结构

```text
foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── other.rs
```

### Cargo.toml

```toml
[package]
name = "rust"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### main.rs

```rust
fn main() {
    println!("main.rs")
}
```

### other.rs

```rust
fn main() {
    println!("other.rs")
}
```

### 默认可执行文件

```text
PS C:\Users\jiangbo1446\work\workspace\rust\rust> cargo run --bin rust
   Compiling rust v0.1.0 (C:\Users\jiangbo1446\work\workspace\rust\rust)
    Finished dev [unoptimized + debuginfo] target(s) in 1.74s
     Running `target\debug\rust.exe`
main.rs
```

### 其它可执行文件

```text
PS C:\Users\jiangbo1446\work\workspace\rust\rust> cargo run --bin other
   Compiling rust v0.1.0 (C:\Users\jiangbo1446\work\workspace\rust\rust)
    Finished dev [unoptimized + debuginfo] target(s) in 1.82s
     Running `target\debug\other.exe`
other.rs
```

## 总结

了解了 Rust 中怎么可以生成多个可执行文件，以及怎么运行。

## 附录
