# 【Axum】修改返回状态码

## 环境

- Time 2022-01-16
- Rust 1.58.0
- Axum 0.4.4

## 概念

参考：<https://docs.rs/axum/latest/axum/index.html>  

## 示例

### main.rs

```rust
use axum::{http::StatusCode, routing::get, Router};
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

async fn home() -> (StatusCode, String) {
    (StatusCode::CREATED, "jiangbo".into())
}
```

## 总结

修改 HTTP 请求的响应码。

## 附录
