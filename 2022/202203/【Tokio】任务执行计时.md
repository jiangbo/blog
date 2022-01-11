# 【Tokio】任务执行计时

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/time/struct.Instant.html>  

## 示例

### main.rs

```rust
use std::{io, time::Duration};

use tokio::time::{self, Instant};

#[tokio::main]
async fn main() -> io::Result<()> {
    let now = Instant::now();

    time::sleep(Duration::from_secs(4)).await;
    println!("elapsed: {} ms", now.elapsed().as_millis());

    Ok(())
}
```

## 总结

使用 `Instant` 来进行任务计时。

## 附录
