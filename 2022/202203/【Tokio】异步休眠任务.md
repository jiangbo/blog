# 【Tokio】异步休眠任务

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/time/index.html>  

标准库和 Tokio 都提供了线程休眠函数 sleep，标准库的休眠会挂起线程，不做任何事。而 Tokio 的可以放弃执行当前任务去执行其它任务，线程并不会被挂起。

## 示例

### 标准库

可以看到先打印 `hello tokio`，等待四秒后再次打印，中间间隔了四秒钟。

```rust
use std::{io, thread, time::Duration};

use tokio::time;

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    tokio::spawn(async {
        println!("hello tokio");
        thread::sleep(Duration::from_secs(4));
    });

    tokio::spawn(async {
        println!("hello tokio");
        thread::sleep(Duration::from_secs(4));
    });

    time::sleep(Duration::from_secs(14)).await;
    Ok(())
}
```

### sleep

可以看到同时打印了 `hello tokio`。

```rust
use std::{io, time::Duration};

use tokio::time;

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    tokio::spawn(async {
        println!("hello tokio");
        time::sleep(Duration::from_secs(4)).await;
    });

    tokio::spawn(async {
        println!("hello tokio");
        time::sleep(Duration::from_secs(4)).await;
    });

    time::sleep(Duration::from_secs(14)).await;
    Ok(())
}
```

### sleep_util

前面的是休眠多久，还可以设置休眠到什么时候。

```rust
use std::{io, time::Duration};

use tokio::time::{self, Instant};

#[tokio::main]
async fn main() -> io::Result<()> {
    time::sleep_until(Instant::now() + Duration::from_secs(4)).await;
    println!("4s have elapsed");
    Ok(())
}
```

## 总结

查看了 Tokio 和标准库中线程休眠函数的不同，以及休眠多久和休眠到什么时候。

## 附录
