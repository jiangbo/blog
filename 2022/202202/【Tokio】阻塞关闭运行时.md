# 【Tokio】阻塞关闭运行时

## 环境

- Time 2022-01-10
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html>  

`shutdown_timeout` 方法关闭运行时会阻塞，等待任务完成。参数可以提供最大等待时间，超时将被强制结束。

## 示例

### main.rs

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Runtime;

fn main() -> io::Result<()> {
    let runtime = Runtime::new()?;

    runtime.spawn(async {
        println!("hello tokio");
        thread::sleep(Duration::from_secs(2));
        println!("{}", thread::current().name().unwrap());
    });

    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

### 结果

运行时关闭的时候，是等待任务完成后才进行关闭的，所以能看到全部的输出。

```text
main
hello tokio
tokio-runtime-worker
```

## 总结

`shutdown_timeout` 方法可以在等待一段时间后再关闭运行时，超时会被强制结束。

## 附录
