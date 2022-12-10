# 0269-GRPC-使用 prost 编码

## 环境

- Time 2022-12-10
- WSL-Ubuntu 22.04
- prost 0.11

## 前言

### 说明

参考：<https://docs.rs/prost-build/latest/prost_build/>

### 目标

在前一节的基础上，使用 prost 进行编码。

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

fn main() {
    let user = proto::User {
        id: 1,
        name: "JiangBo".to_string(),
        age: 44,
    };

    let encode = user.encode_to_vec();
    println!(
        "encode: {encode:?}, encode length: {}, length: {}",
        user.encoded_len(),
        encode.len()
    );
}
```

## 运行结果

```text
encode: [8, 1, 18, 7, 74, 105, 97, 110, 103, 66, 111, 24, 44], encode length: 13, length: 13
```

## 总结

使用 prost 来进行编码，得到了编码后的字节和长度。

## 附录
