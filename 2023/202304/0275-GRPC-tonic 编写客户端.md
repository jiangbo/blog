# 0275-GRPC-tonic 编写客户端

## 环境

- Time 2022-12-11
- WSL-Ubuntu 22.04
- tonic 0.8

## 前言

### 说明

参考：<https://github.com/hyperium/tonic/tree/master/examples/src/helloworld>

### 目标

在上一节的基础上，增加 RPC 调用，编写客户端的代码。

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

// 用户请求
message UserRequest{
}

// 用户响应
message UserResponse{
  repeated User users = 1; // 用户列表
}

// 用户服务
service UserRpc{
  rpc list (UserRequest) returns (UserResponse);
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
tokio = {version = "1", features = ["rt-multi-thread"]}
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

## lib.rs

```Rust
mod proto;

pub use proto::*;
```

## client.rs

```Rust
use game::{user_rpc_client::UserRpcClient, UserRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserRpcClient::connect("http://[::1]:4444").await?;

    let request = tonic::Request::new(UserRequest {});
    let response = client.list(request).await?;

    dbg!(response);
    Ok(())
}
```

## 运行结果

```text
[src/bin/client.rs:10] response = Response {
    metadata: MetadataMap {
        headers: {
            "content-type": "application/grpc",
            "date": "Sun, 11 Dec 2022 05:12:50 GMT",
            "grpc-status": "0",
        },
    },
    message: UserResponse {
        users: [
            User {
                id: 1,
                name: "JiangBo",
                age: 44,
            },
        ],
    },
    extensions: Extensions,
}
```

## 总结

使用 tonic 编写 RPC 调用的客户端代码。

## 附录
