# 【Mio】UDP 服务器

## 环境

- Time 2022-01-08
- Rust 1.57.0
- mio 0.8

## 概念

参考：<https://github.com/tokio-rs/mio/blob/master/examples/tcp_server.rs>  

将客户端发送的消息转为大写后返回。

> UDP 服务器基本功能实现完成，只能做练习使用，不可用于生产环境。

## 示例

### 监听端口

```rust
const SERVER: Token = Token(0);
fn main() -> io::Result<()> {
    let addr = "127.0.0.1:4444".parse().unwrap();
    let mut server = UdpSocket::bind(addr)?;

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            handle(event, &server)?;
        }
    }
}
```

### 处理

```rust
fn handle(event: &Event, server: &UdpSocket) -> io::Result<()> {
    if event.token() != SERVER {
        return Ok(());
    }

    let mut buffer = vec![0; 4096];
    loop {
        match server.recv_from(&mut buffer) {
            Ok((size, address)) => {
                println!("客户端: {}", address);
                let received = &buffer[..size];
                let str = from_utf8(received).unwrap();
                println!("收到数据：{}", str);
                server.send_to(str.to_ascii_uppercase().as_bytes(), address)?;
            }
            Err(e) if e.kind() == WouldBlock => break,
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
```

## 总结

使用 UDP 协议实现了将客户端发送的内容进行大写转换后返回。

## 附录

### 完整代码

```rust
use mio::{event::Event, net::UdpSocket, Events, Interest, Poll, Token};
use std::{
    io::{self, ErrorKind::WouldBlock},
    str::from_utf8,
};

const SERVER: Token = Token(0);
fn main() -> io::Result<()> {
    let addr = "127.0.0.1:4444".parse().unwrap();
    let mut server = UdpSocket::bind(addr)?;

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            handle(event, &server)?;
        }
    }
}

fn handle(event: &Event, server: &UdpSocket) -> io::Result<()> {
    if event.token() != SERVER {
        return Ok(());
    }

    let mut buffer = vec![0; 4096];
    loop {
        match server.recv_from(&mut buffer) {
            Ok((size, address)) => {
                println!("客户端: {}", address);
                let received = &buffer[..size];
                let str = from_utf8(received).unwrap();
                println!("收到数据：{}", str);
                server.send_to(str.to_ascii_uppercase().as_bytes(), address)?;
            }
            Err(e) if e.kind() == WouldBlock => break,
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
```
