# 0274-GRPC-tonic 编写服务端

## 环境

- Time 2022-12-11
- WSL-Ubuntu 22.04
- tonic 0.8

## 前言

### 说明

参考：<https://github.com/hyperium/tonic/tree/master/examples/src/helloworld>

### 目标

在上一节的基础上，增加 RPC 调用，编写服务端的代码。

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

## server.rs

```Rust
use game::user_rpc_server::{UserRpc, UserRpcServer};
use game::{User, UserRequest, UserResponse};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct UserService {}

type UserResult = Result<Response<UserResponse>, Status>;

#[tonic::async_trait]
impl UserRpc for UserService {
    async fn list(&self, _: Request<UserRequest>) -> UserResult {
        let user = User {
            id: 1,
            name: "JiangBo".to_string(),
            age: 44,
        };
        Ok(Response::new(UserResponse { users: vec![user] }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:4444".parse().unwrap();

    println!("listening on {}", addr);

    Server::builder()
        .add_service(UserRpcServer::new(UserService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
```

## 总结

使用 tonic 编写 RPC 调用的服务端代码。

## 附录
