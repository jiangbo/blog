# 【Axum】返回 JSON 文本

## 环境

- Time 2022-01-16
- Rust 1.58.0
- Axum 0.4.4

## 概念

参考：<https://docs.rs/axum/latest/axum/index.html>  

## 示例

### main.rs

```rust
use axum::{routing::get, Json, Router};
use serde_json::Value;
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

async fn home() -> Json<Value> {

    let json: Value = serde_json::from_str(r#"{"id":4,"name":"jiangbo"}"#).unwrap();
    Json(json)
}
```

## 总结

使用 axum 返回 JSON 文本。

## 附录
