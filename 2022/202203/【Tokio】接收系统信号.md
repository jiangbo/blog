# 【Tokio】接收系统信号

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/signal/index.html>  

Tokio 可以异步接收系统传递的信号。

## 示例

### main.rs

按下 `ctrl-c` 时，程序结束。

```rust
use std::{io, thread};

use tokio::signal;

#[tokio::main]
async fn main() -> io::Result<()> {
    tokio::spawn(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
    });

    signal::ctrl_c().await?;
    println!("ctrl-c received");
    Ok(())
}
```

## 总结

Tokio 提供了异步接收系统信号的功能。

## 附录
