# 【Tokio】异步请求响应

## 环境

- Time 2022-01-12
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/sync/index.html>  

## 示例

### main.rs

```rust
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel::<(String, oneshot::Sender<String>)>();

    tokio::spawn(async move {
        while let Some((name, sender)) = rx.recv().await {
            sender.send(format!("hi, {}", name)).unwrap();
        }
    });

    let mut handles = vec![];

    for index in 0..10 {
        let tx = tx.clone();
        handles.push(tokio::spawn(async move {
            let (sender, receiver) = oneshot::channel();
            tx.send((format!("jiangbo-{}", index), sender)).unwrap();
            println!("received: {}", receiver.await.unwrap());
        }));
    }

    for join_handle in handles.drain(..) {
        join_handle.await.unwrap();
    }
}
```

## 总结

使用 `mpsc` 和 `oneshot` 实现异步的请求和响应。

## 附录
