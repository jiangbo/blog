# 【Tokio】异步任务的返回值

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/task/index.html>  

## 示例

### 正常结果

```rust
use std::{io, thread};

#[tokio::main]
async fn main() -> io::Result<()> {
    let result = tokio::spawn(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
        44
    });

    println!("result: {}", result.await?);
    Ok(())
}
```

### 异常结果

```rust
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let result = tokio::spawn(async { panic!("panic") });

    println!("result {}", result.await.is_err());
    Ok(())
}
```

## 总结

Tokio 执行异步任务后，可以有返回值，其中也有可能是出现了错误。

## 附录
