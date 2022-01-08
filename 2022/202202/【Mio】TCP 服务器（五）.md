# 【Mio】TCP 服务器（五）

## 环境

- Time 2022-01-08
- Rust 1.57.0
- mio 0.8

## 概念

参考：<https://github.com/tokio-rs/mio/blob/master/examples/tcp_server.rs>  

实现多个客户端同时连接的功能。

## 示例

### 存储不同的连接

```rust
let mut clients = HashMap::new();
...
clients.insert(unique_token, client);
```

### 获取触发的连接

```rust
if let Some(client) = clients.get_mut(&token) {
    client.write_all(b"hello world\n").unwrap();
}
```

### 使用 nc 命令访问

```text
nc 127.0.0.1 4444
hello world

```

## 总结

使用 mio 编写 TCP 服务器，可以支持多个客户端同时连接。

## 附录

### 完整代码

```rust
use mio::{net::TcpListener, Events, Interest, Poll, Token};
use std::{
    collections::HashMap,
    io::{self, ErrorKind::WouldBlock, Write},
};

const SERVER: Token = Token(0);
fn main() -> io::Result<()> {
    let addr = "127.0.0.1:4444".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    let mut clients = HashMap::new();
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
                    clients.insert(unique_token, client);
                },
                token => {
                    if event.is_writable() {
                        println!("状态可写");

                        if let Some(client) = clients.get_mut(&token) {
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
