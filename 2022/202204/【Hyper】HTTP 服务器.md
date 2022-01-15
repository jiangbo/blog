# 【Hyper】HTTP 服务器

## 环境

- Time 2022-01-15
- Rust 1.58.0
- Tokio 1.15.0
- Hyper 0.14.16

## 概念

参考：<https://docs.rs/hyper/latest/hyper/server/index.html>  

## 示例

### main.rs

```rust
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World")))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));
    let service = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(handle)) });
    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
```

## 总结

使用 Hyper 启动了一个 hello world 服务器。

## 附录
