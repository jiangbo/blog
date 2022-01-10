# 【Tokio】异步运行时

## 环境

- Time 2022-01-10
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html>  

Tokio 是 Rust 的一个异步运行时库，提供了标准库的异步版本，Rust 的异步生态很多都是基于 Tokio 的。

## 示例

### main.rs

```rust
use std::{io, time::Duration};

use tokio::runtime::Runtime;

fn main() -> io::Result<()> {
    let runtime = Runtime::new()?;

    runtime.block_on(async {
        println!("hello tokio");
    });

    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

## 总结

使用 Tokio 作为一个异步运行时，编写了一个 hello world 程序。

## 附录
