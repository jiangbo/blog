# 【Tokio】线程本地运行

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/task/index.html>  

运行阻塞任务可以有两种选择，`spawn_blocking` 和 `block_in_place`。前面说了 `spawn_blocking` 是通过新建一个线程来执行，而 `block_in_place` 是直接在工作线程中运行，避免下上文切换。并且会将当前工作线程上的任务移动到其它工作线程去执行。

## 示例

### main.rs

```rust
use std::{io, thread};

#[tokio::main]
async fn main() -> io::Result<()> {
    tokio::spawn(async {
        tokio::task::block_in_place(|| {
            println!("hello tokio");
            println!("{}", thread::current().name().unwrap());
        });
    }).await?;

    Ok(())
}
```

## 总结

`block_in_place` 是让任务直接在本地线程运行，避免上下文切换。

## 附录
