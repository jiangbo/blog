# 【Tokio】异步周期任务

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/time/index.html>  

## 示例

### interval

```rust
use std::{io, thread, time::Duration};

use tokio::time::{self, Instant};

#[tokio::main]
async fn main() -> io::Result<()> {

let start = Instant::now() + Duration::from_millis(50);
    let mut interval = interval_at(start, Duration::from_millis(200));
    for _i in 0..5 {
        interval.tick().await;
        async {
            println!("{}", thread::current().name().unwrap());
            println!("elapsed: {} ms", now.elapsed().as_millis())
        }
        .await;
    }

    Ok(())
}
```

### interval_at

```rust
use std::{io, thread};

use tokio::time::{self, Duration, Instant};

#[tokio::main]
async fn main() -> io::Result<()> {
    let start = Instant::now() + Duration::from_secs(4);
    // 四秒后，每两百毫秒执行一次
    let mut interval = time::interval_at(start, Duration::from_millis(200));
    for _i in 0..5 {
        interval.tick().await;
        async {
            println!("{}", thread::current().name().unwrap());
        }
        .await;
    }

    Ok(())
}
```

## 总结

通过 `interval` 和 `interval_at` 来定义异步的周期性任务。

## 附录
