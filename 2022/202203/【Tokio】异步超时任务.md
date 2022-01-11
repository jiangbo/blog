# 【Tokio】异步超时任务

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/time/index.html>  

## 示例

### timeout

```rust
use std::io;

use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> io::Result<()> {
    if let Err(e) = time::timeout(Duration::from_secs(1), async {
        time::sleep(Duration::from_secs(4)).await;
    })
    .await
    {
        println!("error: {}", e);
    }

    Ok(())
}
```

### timeout_at

```rust
use tokio::time::{self, Duration, Instant};

#[tokio::main]
async fn main() {
    let deadline = Instant::now() + Duration::from_secs(1);
    if let Err(e) = time::timeout_at(deadline, async {
        time::sleep(Duration::from_secs(4)).await;
    })
    .await
    {
        println!("error: {}", e);
    }
}
```

## 总结

通过 `timeout` 和 `timeout_at` 来定义异步的超时任务。

## 附录
