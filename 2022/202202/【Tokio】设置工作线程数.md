# 【Tokio】设置工作线程数

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html>  

默认情况下，Tokio 启动的工作线程数和 CPU 核数相等，也可以自定义。

## 示例

### main.rs

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Builder;

fn main() -> io::Result<()> {
    let runtime = Builder::new_multi_thread().worker_threads(4).build()?;

    runtime.spawn(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
    });

    println!("{}", thread::current().name().unwrap());
    thread::sleep(Duration::from_secs(4444));
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

## 总结

使用 `Builder` 来定义异步运行时的工作线程数。

## 附录
