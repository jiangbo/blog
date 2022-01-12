# 【Tokio】单次一对一通道

## 环境

- Time 2022-01-12
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html>  

`` 可以实现单对单的消息通信，不过只能通信一次。

## 示例

### main.rs

一次跨线程的通信。

```rust
use std::thread;

use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async {
        println!("{}", thread::current().name().unwrap());
        if tx.send("jiangbo").is_err() {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got = {}", v),
        Err(_) => println!("the sender dropped"),
    }
    println!("{}", thread::current().name().unwrap());
}
```

### JoinHandle

如果是任务的最终结果，除了通过消息通信，也可以直接获取。

```rust
use std::thread;

#[tokio::main]
async fn main() {
    let result = tokio::spawn(async {
        println!("{}", thread::current().name().unwrap());
        "jiangbo"
    });

    match result.await {
        Ok(v) => println!("got = {}", v),
        Err(_) => println!("error"),
    }
    println!("{}", thread::current().name().unwrap());
}
```

### 错误

如果没有发送任何消息就删除了发送者，接收者会异常。

```rust
use std::thread;

use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<i32>();

    tokio::spawn(async {
        println!("{}", thread::current().name().unwrap());
        drop(tx);
    });

    match rx.await {
        Ok(_) => unreachable!(),
        Err(_) => println!("the sender dropped"),
    }
    println!("{}", thread::current().name().unwrap());
}
```

## 总结

`oneshot` 用来实现一次单对单的通信，可以跨线程。

## 附录
