# 【Hyper】设置请求路由

## 环境

- Time 2022-01-15
- Rust 1.58.0
- Tokio 1.15.0
- Hyper 0.14.16

## 概念

参考：<https://hyper.rs/guides/server/echo/>  

## 示例

### main.rs

```rust
use futures::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::net::SocketAddr;

async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new("hello world".into())),
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),
        (&Method::POST, "/upper") => {
            let chunk_stream = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            Ok(Response::new(Body::wrap_stream(chunk_stream)))
        }

        (&Method::POST, "/reverse") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let reversed_body: Vec<u8> = whole_body.iter().rev().cloned().collect();
            Ok(Response::new(Body::from(reversed_body)))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));
    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(route)) });
    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
```

## 总结

使用 Hyper 来定义根据不同的请求类型和路径进行路由。

## 附录
