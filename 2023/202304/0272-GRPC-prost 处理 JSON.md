# 0272-GRPC-prost 处理 JSON

## 环境

- Time 2022-12-10
- WSL-Ubuntu 22.04
- prost 0.11

## 前言

### 说明

参考：<https://docs.rs/prost-build/latest/prost_build/>

### 目标

使用 prost 来处理 JSON 的派生。

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
serde = {version = "1", features = ["derive"]}
serde_json = "1"

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
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
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
#[derive(serde::Serialize, serde::Deserialize)]
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

fn main() -> std::io::Result<()> {
    let user = proto::User {
        id: 1,
        name: "JiangBo".to_string(),
        age: 44,
    };

    let json = serde_json::to_string(&user)?;
    println!("{:?}", json);
    let user: proto::User = serde_json::from_str(&json)?;
    dbg!(user);
    Ok(())
}
```

## 运行结果

```text
"{\"id\":1,\"name\":\"JiangBo\",\"age\":44}"
[src/main.rs:13] user = User {
    id: 1,
    name: "JiangBo",
    age: 44,
}
```

## 总结

使用 prost 来进行 JSON 的处理。

## 附录
