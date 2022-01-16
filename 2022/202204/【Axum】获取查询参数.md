# 【Axum】获取查询参数

## 环境

- Time 2022-01-16
- Rust 1.58.0
- Axum 0.4.4

## 概念

参考：<https://docs.rs/axum/latest/axum/index.html>  

## 示例

### main.rs

```rust
use axum::{extract::Query, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr};

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

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: usize,
    name: String,
}

async fn home(Query(mut params): Query<HashMap<String, String>>) -> String {
    let name = params.remove("name").unwrap();
    println!("{:#?}", name);
    name
}
```

## 总结

获取请求中的查询参数。

## 附录
