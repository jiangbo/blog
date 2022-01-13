# 【Tokio】UDP 服务器

## 环境

- Time 2022-01-13
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/net/struct.UdpSocket.html>  

之前使用 `Mio` 实现了一个简单的 UDP 服务器，再使用 `Tokio` 实现。

>练习使用，不可用于生产环境。

## 示例

### main.rs

```rust
use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = UdpSocket::bind("0.0.0.0:4444").await?;
    let mut buffer = [0; 4096];
    loop {
        let (size, address) = server.recv_from(&mut buffer).await?;
        println!("客户端: {}", address);
        let str = std::str::from_utf8(&buffer[..size]).unwrap();
        println!("收到数据：{}", str);

        server.send_to(str.to_uppercase().as_bytes(), address).await?;
    }
}
```

## 总结

使用 Tokio 实现 UDP 服务器的功能。

## 附录
