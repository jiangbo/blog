# 【Axum】hello world

## 环境

- Time 2022-01-16
- Rust 1.58.0
- Axum 0.4.4

## 概念

参考：<https://github.com/tokio-rs/axum/blob/main/examples/hello-world/src/main.rs>  

## 示例

### main.rs

```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "hello world" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

## 总结

使用 axum 启动了一个服务器，访问时返回 `hello world`。

## 附录
