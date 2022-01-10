# 【Tokio】工作线程任务

## 环境

- Time 2022-01-10
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html>  

spawn 方法可以接收一个异步任务，在工作线程中运行，并不产生阻塞。

## 示例

### 异步运行任务

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Runtime;

fn main() -> io::Result<()> {
    let runtime = Runtime::new()?;

    runtime.spawn(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
    });

    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

### 结果

可以看到运行在不同的线程中，一个是主线程，一个是工作线程。因为存在两个线程，所以输出的顺序不固定。main 可能在最前，也可能在最后。

```text
main
hello tokio
tokio-runtime-worker
```

## 总结

提交给 `spawn` 的任务，会在工作线程中运行，并且是非阻塞的。

## 附录
