# 【Tokio】阻塞线程任务

## 环境

- Time 2022-01-10
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html>  

`spawn_blocking` 方法可以接收一个闭包，可以是一个阻塞任务。

> Tokio 有两种线程。一种给异步任务的核心线程，一种是运行同步任务的阻塞线程。
> 核心线程池的数量和 CPU 核数相同，阻塞线程只有在需要的时候新建。

## 示例

### main.rs

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Runtime;

fn main() -> io::Result<()> {
    let runtime = Runtime::new()?;

    runtime.spawn_blocking(|| {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
    });

    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

## 总结

`spawn_blocking` 方法用来提交阻塞任务。

## 附录
