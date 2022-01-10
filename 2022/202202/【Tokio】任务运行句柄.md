# 【Tokio】任务运行句柄

## 环境

- Time 2022-01-10
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html>  

`handle` 方法返回一个可以执行任务的句柄。

## 示例

### main.rs

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Runtime;

fn main() -> io::Result<()> {
    let runtime = Runtime::new()?;

    let handle = runtime.handle();
    handle.spawn(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
    });

    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

## 总结

`handle` 方法可以获取一个执行任务的句柄。

## 附录
