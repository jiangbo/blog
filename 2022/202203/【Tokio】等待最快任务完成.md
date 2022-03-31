# 【Tokio】等待最快任务完成

## 环境

- Time 2022-01-14
- Rust 1.58.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/macro.select.html>  

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

    tokio::select! {
        first = time_sleep(1) => {
            println!("sleep 1: {first}")
        }
        second = time_sleep(2) => {
            println!("sleep 2: {second}")
        }
    }

    let time = now.elapsed().as_millis();
    println!("elapsed: {time} ms");
}
```

### 输出

等待最快的任务完成就结束。

```text
sleep 1: 1
elapsed: 1001 ms
```

## 总结

`select!` 宏可以等待最快的任务完成。

## 附录
