# 【Tokio】自定义线程名称

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html>  

通过 `thread_name` 和 `thread_name_fn` 可以对线程的名称进行设置。

## 示例

### 字符串

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Builder;

fn main() -> io::Result<()> {
    let runtime = Builder::new_multi_thread()
        .thread_name("jiangbo-worker")
        .build()?;

    runtime.spawn(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
    });

    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

### 函数

```rust
use std::{
    io,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

use tokio::runtime::Builder;

fn main() -> io::Result<()> {
    let runtime = Builder::new_multi_thread()
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("jiangbo-{}", id)
        })
        .build()?;

    (0..4).for_each(|_| {
        runtime.spawn(async {
            println!("hello tokio");
            println!("{}", thread::current().name().unwrap());
        });
    });

    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

## 总结

对于 Tokio 中的线程，可以自定义线程名称。

## 附录
