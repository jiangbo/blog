# 0273-GRPC-tonic 进行编解码

## 环境

- Time 2022-12-11
- WSL-Ubuntu 22.04
- tonic 0.8

## 前言

### 说明

参考：<https://github.com/hyperium/tonic/tree/master/examples/src/helloworld>

### 目标

使用 tonic 来处理 protobuf 编码和解码。

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
tonic = "0.8"

[build-dependencies]
tonic-build = "0.8"
```

## build.rs

```Rust
use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .out_dir("src/proto")
        // 处理 clippy 警告的问题
        .type_attribute(".", "#[derive(Eq)]")
        .compile(&["proto/user.proto"], &["proto/"])?;
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
#[derive(Eq)]
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
use prost::Message;

mod proto;

fn main() -> std::io::Result<()> {
    let user = proto::User {
        id: 1,
        name: "JiangBo".to_string(),
        age: 44,
    };

    let encode = user.encode_to_vec();
    let user = proto::User::decode(encode.as_ref())?;
    dbg!(user);
    Ok(())
}
```

## 运行结果

```text
[src/main.rs:14] user = User {
    id: 1,
    name: "JiangBo",
    age: 44,
}
```

## 总结

使用 tonic 来进行编码和解码的处理。

## 附录
