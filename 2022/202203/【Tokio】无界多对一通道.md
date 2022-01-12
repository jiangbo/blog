# 【Tokio】无界多对一通道

## 环境

- Time 2022-01-12
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/sync/index.html>  

`mpsc` 可以实现多对一的消息通信，表示：Multi-producer, single-consumer。

## 示例

### main.rs

无界通道不会阻塞发送者。

```rust
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel();

    for index in 0..10 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(format!("index {}", index)).unwrap();
            println!("send index: {}", index);
        });
    }

    drop(tx); // 让发送者被回收，不然接收者会一直等待
    while let Some(res) = rx.recv().await {
        println!("got = {}", res);
        time::sleep(Duration::from_secs(1)).await;
    }
}
```

## 总结

`mpsc` 可以实现多个生产者，一个消费者的通道，无界通道不会阻塞发送者，所以发送可以不用异步。

## 附录
