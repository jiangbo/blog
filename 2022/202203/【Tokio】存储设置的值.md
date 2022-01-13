# 【Tokio】存储设置的值

## 环境

- Time 2022-01-13
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://tokio.rs/tokio/tutorial/spawning>  

## 示例

### main.rs

```rust
use mini_redis::Command::{Get, Set};
use mini_redis::{Command, Connection, Frame};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, address) = listener.accept().await?;
        println!("客户端: {}", address);
        // 提交任务
        tokio::spawn(process(socket)).await??;
    }
}

async fn process(socket: TcpStream) -> mini_redis::Result<()> {
    let mut database = HashMap::new();
    // Connection 是 mini redis 定义的内容
    let mut client = Connection::new(socket);

    // 循环接收数据
    while let Some(frame) = client.read_frame().await? {
        let response = match Command::from_frame(frame)? {
            Set(cmd) => {
                database.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = database.get(cmd.key()) {
                    // 使用 into 转成 Bytes 类型
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        client.write_frame(&response).await.unwrap();
    }

    Ok(())
}
```

## 总结

可以设置客户端发送的值，并且通过命令获取到。不过数据是客户端隔离的，每个客户端自己只能看到自己的数据。

## 附录
