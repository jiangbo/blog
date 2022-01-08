# 【Mio】TCP 服务器（六）

## 环境

- Time 2022-01-08
- Rust 1.57.0
- mio 0.8

## 概念

参考：<https://github.com/tokio-rs/mio/blob/master/examples/tcp_server.rs>  

main 函数越来越长，将长函数分割成功能较少的小函数。

## 示例

### accept

```rust
fn accept(
    server: &TcpListener,
    unique_token: &mut Token,
    poll: &Poll,
) -> io::Result<Option<(Token, TcpStream)>> {
    let (mut client, address) = match server.accept() {
        Ok((client, address)) => (client, address),
        Err(e) if e.kind() == WouldBlock => return Ok(None),
        Err(e) => return Err(e),
    };

    unique_token.0 += 1;
    let token = *unique_token;
    poll.registry().register(
        &mut client,
        token,
        Interest::READABLE.add(Interest::WRITABLE),
    )?;

    println!("客户端: {}", address);
    Ok(Some((token, client)))
}
```

### 客户端处理

```rust
fn handle_client(event: &Event, client: &mut TcpStream) {
    if event.is_writable() {
        println!("状态可写");
        client.write_all(b"hello world\n").unwrap();
    }

    if event.is_readable() {
        println!("is_readable")
    }
}
```

### 使用 nc 命令访问

```text
nc 127.0.0.1 4444
hello world

```

## 总结

分割 main 函数，将长函数分割成小函数。

## 附录

### 完整代码

```rust
use mio::{
    event::Event,
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};
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
                SERVER => {
                    while let Some((token, client)) = accept(&server, &mut unique_token, &poll)? {
                        clients.insert(token, client);
                    }
                }
                token => {
                    if let Some(client) = clients.get_mut(&token) {
                        handle_client(event, client);
                    }
                }
            }
        }
    }
}

fn accept(
    server: &TcpListener,
    unique_token: &mut Token,
    poll: &Poll,
) -> io::Result<Option<(Token, TcpStream)>> {
    let (mut client, address) = match server.accept() {
        Ok((client, address)) => (client, address),
        Err(e) if e.kind() == WouldBlock => return Ok(None),
        Err(e) => return Err(e),
    };

    unique_token.0 += 1;
    let token = *unique_token;
    poll.registry().register(
        &mut client,
        token,
        Interest::READABLE.add(Interest::WRITABLE),
    )?;

    println!("客户端: {}", address);
    Ok(Some((token, client)))
}

fn handle_client(event: &Event, client: &mut TcpStream) {
    if event.is_writable() {
        println!("状态可写");
        client.write_all(b"hello world\n").unwrap();
    }

    if event.is_readable() {
        println!("is_readable")
    }
}
```
