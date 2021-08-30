# 【Rust】Cargo 编译和运行

## 环境

- Windows 10
- Rust 1.54.0

## Hello World

根据传统，首先编写一个 Hello World 程序。  
参考这里的代码：<https://doc.rust-lang.org/cargo/getting-started/first-steps.html>

> Cargo 是 Rust 的包管理器，和 Java 中的 Maven 类似。

### 建立目录结构

首先建立一个项目目录：hello_world，然后在目录中新建 Cargo.toml 和 src 文件夹，  
最后在 src 文件夹下新建一个 main.rs 的文件，目录结构如下：

```txt
├── Cargo.toml
└── src
    └── main.rs
```

### main.rs 内容

```rs
fn main() {
    println!("Hello World");
}
```

### Cargo.toml 内容

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2018"

[dependencies]
```

### 编译源代码

编译好之后，默认在 `target/debug/` 目录下生成了一个可执行的二进制文件：hello_world.exe。

```txt
C:\Users\jiangbo\work\workspace\rust\hello_world>cargo build
   Compiling hello_world v0.1.0 (C:\Users\jiangbo\work\workspace\rust\hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
```

### 运行程序

```txt
C:\Users\jiangbo\work\workspace\rust\hello_world>target\debug\hello_world.exe
Hello World
```

### 使用 Cargo 运行程序

```txt
C:\Users\jiangbo\work\workspace\rust\hello_world>cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\hello_world.exe`
Hello World
```

## 总结

编写了一个 Rust 程序，使用了 Cargo 的目录结构，并且进行了编译和运行。

## 附录
