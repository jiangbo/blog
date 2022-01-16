# 【Axum】返回 JSON 对象

## 环境

- Time 2022-01-16
- Rust 1.58.0
- Axum 0.4.4

## 概念

参考：<https://docs.rs/axum/latest/axum/index.html>  

## 示例

### toml

```toml
[package]
edition = "2021"
name = "game"
version = "0.1.0"

[dependencies]
axum = "*"
serde = {version = "*", features = ["derive"]}
serde_json = "*"
tokio = {version = "*", features = ["full"]}
```

### main.rs

```rust
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct User {
    id: usize,
    name: String,
}

async fn home() -> Json<User> {
    Json(User {
        id: 4,
        name: "jiangbo".into(),
    })
}
```

## 总结

使用 axum 返回 JSON 对象。

## 附录
