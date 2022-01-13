# 【Tokio】提交异步任务

## 环境

- Time 2022-01-13
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://tokio.rs/tokio/tutorial/spawning>  

## 示例

### main.rs

```rust
use mini_redis::{Connection, Frame};
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
    // Connection 是 mini redis 定义的内容
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await? {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
    Ok(())
}
```

## 总结

分离接收客户端和处理客户端传递的数据。

## 附录
