# 【Tokio】单线程运行时

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html>  

除了可以使用线程池运行时，还可以直接在当前线程运行任务，使用单线程。

## 示例

### main.rs

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Builder;

fn main() -> io::Result<()> {
    let runtime = Builder::new_current_thread().build()?;

    runtime.block_on(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
    });

    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

## 总结

新建了一个单线程的异步运行时。

## 附录
