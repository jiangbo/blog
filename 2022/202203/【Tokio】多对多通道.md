# 【Tokio】多对多通道

## 环境

- Time 2022-01-12
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/sync/index.html>  

## 示例

### main.rs

```rust
use std::time::Duration;

use tokio::{sync::broadcast, time};

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        println!("rx1: {}", rx1.recv().await.unwrap());
        println!("rx1: {}", rx1.recv().await.unwrap());
    });

    tokio::spawn(async move {
        println!("rx2: {}", rx2.recv().await.unwrap());
        println!("rx2: {}", rx2.recv().await.unwrap());
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
    time::sleep(Duration::from_secs(1)).await;
}
```

### 延时错误

```rust
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(2);

    tx.send(10).unwrap();
    tx.send(20).unwrap();
    tx.send(30).unwrap();

    if let Err(e) = rx.recv().await {
        println!("error: {}", e);
    }

    println!("rx: {}", rx.recv().await.unwrap());
    println!("rx: {}", rx.recv().await.unwrap());
}
```

## 总结

`broadcast` 可以实现多对多的通道，消息只有在所有的接收者受到后才删除。

## 附录
