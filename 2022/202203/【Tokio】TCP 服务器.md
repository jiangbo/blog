# 【Tokio】TCP 服务器

## 环境

- Time 2022-01-13
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/index.html>  

之前使用 `Mio` 实现了一个简单的 TCP 服务器，再使用 `Tokio` 实现。

>练习使用，不可用于生产环境。

## 示例

### main.rs

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4444").await?;

    loop {
        let (client, address) = listener.accept().await?;
        println!("客户端: {}", address);
        tokio::spawn(process_socket(client));
    }
}

async fn process_socket(mut client: TcpStream) -> Result<()> {
    let mut buffer = vec![0; 4096];
    loop {
        let size = client.read(&mut buffer).await?;
        if size == 0 {
            println!("{} 连接已关闭", client.peer_addr()?);
            return Ok(());
        }

        let str = std::str::from_utf8(&buffer[..size]).unwrap();
        println!("收到数据：{}", str);
        client.write_all(str.to_uppercase().as_bytes()).await?;
    }
}
```

## 总结

使用 Tokio 实现 TCP 服务器的功能。

## 附录
