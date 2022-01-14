# 【Tokio】等待所有任务完成

## 环境

- Time 2022-01-14
- Rust 1.58.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/macro.join.html>  

## 示例

### main.rs

```rust
use std::time::Duration;

use tokio::time::{sleep, Instant};

async fn time_sleep(time: u64) -> u64 {
    sleep(Duration::from_secs(time)).await;
    time
}

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let (first, second) = tokio::join!(time_sleep(1), time_sleep(2));
    let time = now.elapsed().as_millis();
    println!("elapsed: {time} ms, first: {first}, second: {second}");
}
```

### 输出

等待最久的任务完成后才结束。

```text
elapsed: 2001 ms, first: 1, second: 2
```

## 总结

`join!` 宏可以等待所有任务完成。

## 附录
