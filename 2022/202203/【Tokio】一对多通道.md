# 【Tokio】一对多通道

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

use tokio::{sync::watch, time};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel("hello");

    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            println!("received = {:?}", *rx.borrow());
        }
    });

    tx.send("world").unwrap();
    time::sleep(Duration::from_secs(1)).await;
}
```

## 总结

`watch` 可以实现一对多的通道。

## 附录
