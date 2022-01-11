# 【Tokio】宏创建运行时

## 环境

- Time 2022-01-11
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/attr.main.html>  

除了使用明确编写代码来新建运行时，也可以使用宏来创建。

## 示例

### main.rs

```rust
use std::{io, thread};

#[tokio::main]
async fn main() -> io::Result<()> {
    tokio::spawn(async {
        println!("hello tokio");
        println!("{}", thread::current().name().unwrap());
    });

    println!("{}", thread::current().name().unwrap());
    Ok(())
}
```

### 多线程

```rust
#[tokio::main(flavor = "multi_thread")]
```

### 当前线程

```rust
#[tokio::main(flavor = "current_thread")]
```

### 工作线程数

```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
```

## 总结

可以使用宏来创建 Tokio 的异步运行时。

## 附录
