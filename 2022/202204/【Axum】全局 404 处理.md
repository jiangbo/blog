# 【Axum】全局 404 处理

## 环境

- Time 2022-01-16
- Rust 1.58.0
- Axum 0.4.4

## 概念

参考：<https://github.com/tokio-rs/axum/blob/main/examples/global-404-handler>  

## 示例

### main.rs

```rust
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{handler::Handler, routing::get, Json, Router};
use serde_json::json;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .fallback(handler_404.into_service());
    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> String {
    "jiangbo".into()
}

async fn handler_404() -> impl IntoResponse {
    let json = json!({
        "code": "01010101",
        "msg": "路径不存在"
    });

    (StatusCode::NOT_FOUND, Json(json))
}
```

## 总结

定义全局的 404 处理。

## 附录
