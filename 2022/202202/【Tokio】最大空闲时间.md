# 【Tokio】最大空闲时间

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html>  

对于阻塞任务，Tokio 会新启动一个线程来运行，这个也是在一个线程池中，任务完成后，不会立即销毁。经过空余时间后，还是没有任务，就会进行销毁，默认 10 秒。

## 示例

### main.rs

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Builder;

fn main() -> io::Result<()> {
    let runtime = Builder::new_multi_thread()
        .thread_keep_alive(Duration::from_secs(14))
        .build()?;

    runtime.spawn_blocking(|| {
        println!("hello tokio");
        thread::sleep(Duration::from_secs(4));
    });

    thread::sleep(Duration::from_secs(44));
    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

## 总结

使用 `thread_keep_alive` 方法来定义阻塞线程的最大空闲时间。

## 附录
