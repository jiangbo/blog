# 【Tokio】有界多对一通道

## 环境

- Time 2022-01-12
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/sync/index.html>  

`mpsc` 可以实现多对一的消息通信，表示：Multi-producer, single-consumer。

## 示例

### main.rs

发送通道满的时候，会被阻塞。

```rust
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::{self, Instant};

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let (tx, mut rx) = mpsc::channel(4);

    for index in 0..10 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(format!("index {}", index)).await.unwrap();
            println!("send index: {}", index);
        });
    }

    drop(tx); // 让发送者被回收，不然接收者会一直等待
    while let Some(res) = rx.recv().await {
        println!("got = {}", res);
        time::sleep(Duration::from_secs(1)).await;
    }
    println!("elapsed {} ms", now.elapsed().as_millis())
}
```

## 总结

`mpsc` 可以实现多个生产者，一个消费者的通道，可以对通道设置上限，达到最大时会被阻塞。

## 附录
