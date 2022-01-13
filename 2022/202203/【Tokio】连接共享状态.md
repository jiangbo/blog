# 【Tokio】连接共享状态

## 环境

- Time 2022-01-13
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://tokio.rs/tokio/tutorial/shared-state>  

> 如果使用 `#[tokio::main(flavor = "current_thread")]` 单线程模型就不需要锁。

## 示例

### Cargo.toml

```toml
[package]
edition = "2021"
name = "game"
version = "0.1.0"

[dependencies]
bytes = "*"
mini-redis = "*"
tokio = {version = "*", features = ["full"]}
```

### main.rs

```rust
use bytes::Bytes;
use mini_redis::Command::{Get, Set};
use mini_redis::{Command, Connection, Frame, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::net::{TcpListener, TcpStream};

type DatabaseLock = Arc<Mutex<HashMap<String, Bytes>>>;
type Database<'a> = MutexGuard<'a, HashMap<String, Bytes>>;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let database: DatabaseLock = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, address) = listener.accept().await?;
        println!("客户端: {}", address);
        // 提交任务
        tokio::spawn(process(socket, database.clone())).await??;
    }
}

async fn process(socket: TcpStream, db_lock: DatabaseLock) -> Result<()> {
    // Connection 是 mini redis 定义的内容
    let mut client = Connection::new(socket);

    // 循环接收数据
    while let Some(frame) = client.read_frame().await? {
        let response = match Command::from_frame(frame)? {
            Set(cmd) => {
                let mut database: Database = db_lock.lock().unwrap();
                database.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let database: Database = db_lock.lock().unwrap();
                if let Some(value) = database.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            _ => Frame::Null,
        };

        client.write_frame(&response).await.unwrap();
    }

    Ok(())
}
```

## 总结

实现了跨客户端连接共享数据，使用的是锁原语。

## 附录
