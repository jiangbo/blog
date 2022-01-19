# 【Axum】全局超时异常

## 环境

- Time 2022-01-16
- Rust 1.58.0
- Axum 0.4.4

## 概念

参考：<https://docs.rs/axum/latest/axum/index.html>  

## 示例

### main.rs

```rust
use axum::error_handling::HandleErrorLayer;
use axum::{http::StatusCode, routing::get, BoxError, Router};
use std::{net::SocketAddr, time::Duration};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_timeout_error))
        .timeout(Duration::from_secs(4));
    let app = Router::new().route("/", get(home)).layer(layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> String {
    tokio::time::sleep(Duration::from_secs(10)).await;
    "hello world".to_owned()
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "timeout".to_string())
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("error: {err}"))
    }
}
```

## 总结

定义请求超时时间，处理请求超时错误。

## 附录
