# 【Tokio】放弃执行时间片

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/task/index.html>  

`yield_now` 可以放弃当前的执行时间片。

## 示例

### main.rs

```rust
use std::{io, thread};

#[tokio::main]
async fn main() -> io::Result<()> {
    tokio::spawn(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
    })
    .await?;

    tokio::task::yield_now().await;
    Ok(())
}
```

## 总结

`yield_now` 可以放弃当前的执行时间片。

## 附录
