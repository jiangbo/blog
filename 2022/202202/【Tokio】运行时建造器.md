# 【Tokio】运行时建造器

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html>  

除了使用 new 直接创建的方式，还可以使用 Builder 来构建运行时，并且提供了更多的配置。

## 示例

### main.rs

```rust
use std::{io, thread, time::Duration};

use tokio::runtime::Builder;

fn main() -> io::Result<()> {
    // 和 let runtime = Runtime::new()?; 具有一样的功能。
    let runtime = Builder::new_multi_thread().build()?;

    runtime.spawn(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
    });

    println!("{}", thread::current().name().unwrap());
    runtime.shutdown_timeout(Duration::from_secs(4));
    Ok(())
}
```

## 总结

`Builder` 可以用来创建一个异步运行时，推荐使用构造器模式而不是直接新建。

## 附录
