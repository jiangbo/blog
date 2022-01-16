# 【Axum】获取路径参数

## 环境

- Time 2022-01-16
- Rust 1.58.0
- Axum 0.4.4

## 概念

参考：<https://docs.rs/axum/latest/axum/index.html>  

## 示例

### main.rs

```rust
use axum::{extract::Path, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:name", get(home));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: usize,
    name: String,
}

async fn home(Path(name): Path<String>) -> String {
    println!("{:#?}", name);
    name
}
```

### 多个路径参数

```rust
use axum::{extract::Path, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:name/:id", get(home));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: usize,
    name: String,
}

async fn home(Path((name, id)): Path<(String, usize)>) -> String {
    format!("name: {name}, id: {id}")
}
```

## 总结

获取请求中的路径参数。

## 附录
