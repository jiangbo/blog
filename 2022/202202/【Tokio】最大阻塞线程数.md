# 【Tokio】最大阻塞线程数

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html>  

对于阻塞任务，Tokio 会新启动一个线程来运行，可以设置启动的最大线程数，默认是 512。

## 示例

### main.rs

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Builder;

fn main() -> io::Result<()> {
    let runtime = Builder::new_multi_thread()
        .max_blocking_threads(4)
        .build()?;

    (0..14).for_each(|index| {
        runtime.spawn_blocking(move || {
            println!("hello tokio {}", index);
            thread::sleep(Duration::from_secs(4));
        });
    });

    // 不立即关闭运行时，不然所有的线程都会运行任务，包括工作线程
    thread::sleep(Duration::from_secs(44));
    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(44));
    Ok(())
}
```

## 总结

使用 `max_blocking_threads` 方法来定义最大的阻塞任务线程数。

## 附录
