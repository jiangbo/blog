# 【mio】TCP 服务器（四）

## 环境

- Time 2022-01-07
- Rust 1.57.0
- mio 0.8

## 概念

参考：<https://github.com/tokio-rs/mio/blob/master/examples/tcp_server.rs>  

在可写事件发生后，向客户端写入一条消息。

## 示例

### 保留连接

```rust
let mut stream = None;

loop {
    poll.poll(&mut events, None)?;
    for event in events.iter() {
        match event.token() {
            SERVER => loop {
                ...
                println!("客户端: {}", address);
                stream = Some(client);
            },
        }
    }
}
```

### 写入数据

```rust
if event.is_writable() {
    println!("状态可写");

    if let Some(mut client) = stream.as_ref() {
        client.write_all(b"hello world\n").unwrap();
    }
}
```

### 使用 nc 命令访问

```text
nc 127.0.0.1 4444
hello world

```

## 总结

在建立连接后，向客户端写入数据。

## 附录

### 完整代码

```rust
use mio::{net::TcpListener, Events, Interest, Poll, Token};
use std::io::{self, ErrorKind::WouldBlock, Write};

const SERVER: Token = Token(0);
fn main() -> io::Result<()> {
    let addr = "127.0.0.1:4444".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    let mut stream = None;
    let mut unique_token = SERVER;

    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    let (mut client, address) = match server.accept() {
                        Ok((client, address)) => (client, address),
                        Err(e) if e.kind() == WouldBlock => break,
                        Err(e) => return Err(e),
                    };

                    unique_token.0 += 1;
                    poll.registry().register(
                        &mut client,
                        unique_token,
                        Interest::READABLE.add(Interest::WRITABLE),
                    )?;

                    println!("客户端: {}", address);
                    stream = Some(client);
                },
                _token => {
                    if event.is_writable() {
                        println!("状态可写");

                        if let Some(mut client) = stream.as_ref() {
                            client.write_all(b"hello world\n").unwrap();
                        }
                    }

                    if event.is_readable() {
                        println!("is_readable")
                    }
                }
            }
        }
    }
}
```
