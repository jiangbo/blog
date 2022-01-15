# 【Hyper】发送 GET 请求

## 环境

- Time 2022-01-15
- Rust 1.58.0
- Tokio 1.15.0
- Hyper 0.14.16

## 概念

参考：<https://hyper.rs/guides/client/basic/>  

## 示例

### main.rs

```rust
use std::error::Error;

use hyper::{body::HttpBody, Client};
use tokio::io::{stdout, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let uri = "http://127.0.0.1:4444".parse()?;
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
```

### 输出

使用客户端连接编写的 HTTP 服务器。

```text
Response: 200 OK
Hello World
```

## 总结

使用 Hyper 发送 GET 请求。

## 附录
