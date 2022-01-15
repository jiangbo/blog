# 【Hyper】发送 POST 请求

## 环境

- Time 2022-01-15
- Rust 1.58.0
- Tokio 1.15.0
- Hyper 0.14.16

## 概念

参考：<https://hyper.rs/guides/client/advanced/>  

## 示例

### main.rs

```rust
use std::error::Error;

use hyper::{body, Body, Client, Method, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://127.0.0.1:4444/reverse")
        .body(Body::from("JiangBo"))?;

    let client = Client::new();
    let resp = client.request(req).await?;

    if resp.status().is_success() {
        let bytes = body::to_bytes(resp.into_body()).await?;
        println!("body: {:?}", bytes);
    }

    Ok(())
}
```

## 总结

使用 Hyper 发送 POST 请求。

## 附录
