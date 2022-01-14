# 【Tokio】客户端使用通道

## 环境

- Time 2022-01-13
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://tokio.rs/tokio/tutorial/channels>  

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

### client

```rust
use mini_redis::{client, Result};

use bytes::Bytes;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot;

type Responder<T> = oneshot::Sender<Result<T>>;
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}
#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(44);
    tokio::spawn(manager(rx));
    task(tx).await?;
    Ok(())
}

async fn manager(mut rx: Receiver<Command>) -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::Get { key, resp } => {
                let res = client.get(&key).await;
                let _ = resp.send(res);
            }
            Command::Set { key, val, resp } => {
                let res = client.set(&key, val).await;
                let _ = resp.send(res);
            }
        }
    }
    Ok(())
}

async fn task(tx: Sender<Command>) -> Result<()> {
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };

        tx.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "hello".to_string(),
            val: "world".into(),
            resp: resp_tx,
        };

        tx2.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });
    t1.await?;
    t2.await?;
    Ok(())
}
```

### server

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

实现了客户端使用一个连接，向服务端发送数据。

## 附录
