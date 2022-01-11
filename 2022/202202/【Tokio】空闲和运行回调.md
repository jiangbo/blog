# 【Tokio】空闲和运行回调

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html>  

在线程空闲和运行时，可以定义一个回调。

## 示例

### main.rs

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Builder;

fn main() -> io::Result<()> {
    let runtime = Builder::new_multi_thread()
        .on_thread_park(|| println!("thread park"))
        .on_thread_unpark(|| println!("thread unpark"))
        .build()?;

    runtime.spawn(async {
        println!("hello tokio");
    });

    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

## 总结

在线程空闲和运行的时候，可以定义一个回调函数。

## 附录
