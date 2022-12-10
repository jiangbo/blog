# 0271-GRPC-prost 带长度的编解码

## 环境

- Time 2022-12-10
- WSL-Ubuntu 22.04
- prost 0.11

## 前言

### 说明

参考：<https://docs.rs/prost-build/latest/prost_build/>

### 目标

在前一节的基础上，使用 prost 进行带长度的编解码。

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
use prost::Message;

mod proto;

fn main() -> std::io::Result<()> {
    let user = proto::User {
        id: 1,
        name: "JiangBo".to_string(),
        age: 44,
    };

    let encode = user.encode_length_delimited_to_vec();
    let user = proto::User::decode_length_delimited(encode.as_ref())?;
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

使用 prost 来进行带长度的解码。

## 附录
