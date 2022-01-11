# 【Tokio】启用新功能

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html>  

Tokio 除了是一个异步运行时外，还提供了其它方面的功能，可以进行开启。

1. `enable_io` 开启 IO 方面的功能
2. `enable_time` 开启时间方面的功能
3. `enable_all` 都开启

## 示例

### main.rs

```rust
use std::{io, thread, time::Duration};

use tokio::{runtime::Builder, time};

fn main() -> io::Result<()> {
    let runtime = Builder::new_current_thread().enable_time().build()?;

    runtime.block_on(async {
        println!("hello tokio");
        // 如果不使用 enable_time 方法，这里会出错
        time::sleep(Duration::from_millis(100)).await;
        println!("{}", thread::current().name().unwrap());
    });

    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

## 总结

使用 `Builder` 时可以控制启用某些功能。

## 附录
