# 【Tokio】进入运行时上下文

## 环境

- Time 2022-01-10
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html>  

`enter` 方法可以进入异步运行时的上下文。

## 示例

### main.rs

```rust
use std::{io, time::Duration};

use tokio::runtime::Runtime;

fn main() -> io::Result<()> {
    let runtime = Runtime::new()?;

    runtime.spawn(hello());
    // 删除它将导致异常
    let _guard = runtime.enter();
    // 如果没有前面的进入异步运行时的上下文，将会失败
    tokio::spawn(hello());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}

async fn hello() {
    println!("hello tokio");
}
```

## 总结

`enter` 方法可以进入异步运行时的上下文。

## 附录
