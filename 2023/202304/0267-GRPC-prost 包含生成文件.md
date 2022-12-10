# 0267-GRPC-prost 包含生成文件

## 环境

- Time 2022-12-10
- WSL-Ubuntu 22.04
- prost 0.11

## 前言

### 说明

参考：<https://docs.rs/prost-build/latest/prost_build/>

### 目标

使用 prost 来自动生成 proto 文件定义的内容，并使用 include 包含进来。

## 安装命令

`apt install protobuf-compiler`

## user.proto 文件

```proto
syntax = "proto3";
package user;
// 用户
message User{
  uint64 id = 1; // 编号
  string name = 2; // 用户名
  uint32 age = 3; // 年龄
}
```

## Cargo.toml

```toml
[package]
edition = "2021"
name = "game"
version = "1.0.0"

[dependencies]
prost = "0.11"

[build-dependencies]
prost-build = "0.11"
```

## build.rs

```Rust
use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["proto/user.proto"], &["proto/"])?;
    Ok(())
}
```

## main.rs

```Rust
fn main() {
    let user = User {
        id: 1,
        name: "JiangBo".to_string(),
        age: 44,
    };

    println!("{user:#?}");
}

include!(concat!(env!("OUT_DIR"), "/user.rs"));
```

## 项目结构

```text
root@jiangbo12490:~/git/game# tree -L 2 .
.
├── Cargo.lock
├── Cargo.toml
├── build.rs
├── proto
│   └── user.proto
├── src
│   └── main.rs
├── target
│   ├── CACHEDIR.TAG
│   ├── debug
│   ├── release
│   └── tmp
└── todo.md
```

## 运行结果

```text
root@jiangbo12490:~/git/game# cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/game`
User {
    id: 1,
    name: "JiangBo",
    age: 44,
}
```

## 总结

使用 prost 来解析 proto 文件，生成对应的 Rust 代码。

## 附录
