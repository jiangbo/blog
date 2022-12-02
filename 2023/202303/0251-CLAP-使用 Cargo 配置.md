# 0251-CLAP-使用 Cargo 配置

## 环境

- Time 2022-12-02
- WSL-Ubuntu 22.04
- CLAP 4.0.29

## 前言

### 说明

参考：<https://docs.rs/clap/latest/clap/index.html>

### 目标

使用 Cargo.toml 的配置来提供命令行的信息。

## Cargo.toml

```toml
[package]
description = "测试程序"
edition = "2021"
name = "game"
version = "1.0.0"

[dependencies]
clap = {version = "4", features = ["cargo"]}
```

## main.rs

```Rust
use clap::{arg, command};

fn main() {
    let matches = command!()
        .arg(arg!(--name <VALUE>).help("姓名"))
        .get_matches();

    if let Some(param) = matches.get_one::<String>("name") {
        println!("输入的姓名是: {}", param);
    }
}
```

## 查看帮助

```text
root@jiangbo12490:~/git/game/target/release# ./game -h
测试程序

Usage: game [OPTIONS]

Options:
      --name <VALUE>  姓名
  -h, --help          Print help information
  -V, --version       Print version information
```

## 查看版本

```text
root@jiangbo12490:~/git/game/target/release# ./game -V
game 1.0.0
```

## 总结

使用 Cargo.toml 的配置来提供命令行的信息。

## 附录
