# 【Tokio】非阻塞关闭运行时

## 环境

- Time 2022-01-10
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html>  

`shutdown_background` 方法可以立即关闭运行时，不会产生阻塞。通常在另一个运行时中调用，避免阻塞。

> 由于 `shutdown_background` 不等待任务结束，可能会产生资源泄露。

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
    runtime.shutdown_background();
    Ok(())
}
```

### 结果

没有看到异步任务的输出，直接结束程序了。

```text
main
```

## 总结

`shutdown_background` 会立即结束运行时，避免阻塞，一般在其他的运行时中调用。

## 附录
