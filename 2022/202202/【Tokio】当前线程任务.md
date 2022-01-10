# 【Tokio】当前线程任务

## 环境

- Time 2022-01-10
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html>  

block_on 方法可以接收一个异步任务，在当前线程中运行并阻塞直到完成。

> `Runtime::new()` 创建的运行时，会有一个主线程和 CPU 逻辑核数相等工作线程。

## 示例

### 运行任务

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Runtime;

fn main() -> io::Result<()> {
    let runtime = Runtime::new()?;

    runtime.block_on(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
    });

    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

### 结果

可以看到都是在主线程中运行的，线程的名称都是 main。

```text
hello tokio
main
main
```

### 带返回值的任务

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Runtime;

fn main() -> io::Result<()> {
    let runtime = Runtime::new()?;

    let result = runtime.block_on(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
        44
    });

    println!("{}", result);
    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

## 总结

提交给 `block_on` 的任务，会在主线程中运行，并且会一直阻塞，直到任务完成才往下执行。

## 附录
