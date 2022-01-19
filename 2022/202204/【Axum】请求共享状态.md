# 【Axum】请求共享状态

## 环境

- Time 2022-01-19
- Rust 1.58.0
- Axum 0.4.4

## 概念

参考：<https://docs.rs/axum/latest/axum/index.html>  

## 示例

### main.rs

```rust
use axum::extract::Extension;
use axum::AddExtensionLayer;
use axum::{routing::get, Router};
use std::{net::SocketAddr, sync::Arc};

struct State {
    name: String,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(State {
        name: "JiangBo".to_owned(),
    });
    let app = Router::new()
        .route("/", get(home))
        .layer(AddExtensionLayer::new(shared_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home(Extension(state): Extension<Arc<State>>) -> String {
    state.name.clone()
}
```

## 总结

通过定义一个共享的变量，在每个请求中共享状态。

## 附录
