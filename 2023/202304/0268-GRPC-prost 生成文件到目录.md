# 0268-GRPC-prost 生成文件到目录

## 环境

- Time 2022-12-10
- WSL-Ubuntu 22.04
- prost 0.11

## 前言

### 说明

参考：<https://docs.rs/prost-build/latest/prost_build/>

### 目标

使用 prost 来自动生成 proto 文件定义的内容，并生成到源码目录。

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

use prost_build::Config;
fn main() -> Result<()> {
    Config::new()
        .out_dir("src/proto")
        .compile_protos(&["proto/user.proto"], &["proto/"])?;
    Ok(())
}
```

## mod.rs

```Rust
mod user;

pub use user::*;
```

## user.rs

```Rust
/// 用户
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// 编号
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// 用户名
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 年龄
    #[prost(uint32, tag = "3")]
    pub age: u32,
}
```

## main.rs

```Rust
mod proto;

fn main() {
    let user = proto::User {
        id: 1,
        name: "JiangBo".to_string(),
        age: 44,
    };

    println!("{user:#?}");
}
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
│   ├── main.rs
│   └── proto
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

使用 prost 来解析 proto 文件，并将源码直接生成到源码目录。

## 附录
