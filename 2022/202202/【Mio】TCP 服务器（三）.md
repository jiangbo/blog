# 【mio】TCP 服务器（三）

## 环境

- Time 2022-01-07
- Rust 1.57.0
- mio 0.8

## 概念

参考：<https://github.com/tokio-rs/mio/blob/master/examples/tcp_server.rs>  

连接建立后，得到一个 socket，使用 epoll 监听可读可写事件。

## 示例

### 获取客户端

```rust
let (client, address) = match server.accept() {
    Ok((client, address)) => (client, address),
    Err(e) if e.kind() == WouldBlock => break,
    Err(e) => return Err(e),
};

println!("客户端: {}", address);
```

### 监听可读可写事件

```rust
_token => {
    if event.is_writable() {
        println!("is_writable")
    }
    if event.is_readable() {
        println!("is_readable")
    }
}
```

### 区分不同事件

```rust
let mut unique_token = SERVER;

loop {
    poll.poll(&mut events, None)?;
    for event in events.iter() {
        match event.token() {
            SERVER => loop {
                ...
                println!("客户端: {}", address);
                unique_token.0 += 1;

                // 注册可读可写监听
                poll.registry().register(
                    &mut client,
                    unique_token,
                    Interest::READABLE.add(Interest::WRITABLE),
                )?;
            },
            _token => {
                ...
            }
        }
    }
}
```

### 使用 nc 命令访问

```text
nc 127.0.0.1 4444
客户端: 127.0.0.1:55348
```

## 总结

建立和客户端的连接，并监听可读可写事件。

## 附录

### 完整代码

```rust
use mio::{net::TcpListener, Events, Interest, Poll, Token};
use std::io::{self, ErrorKind::WouldBlock};

const SERVER: Token = Token(0);
fn main() -> io::Result<()> {
    let addr = "127.0.0.1:4444".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

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

                    println!("客户端: {}", address);
                    unique_token.0 += 1;

                    poll.registry().register(
                        &mut client,
                        unique_token,
                        Interest::READABLE.add(Interest::WRITABLE),
                    )?;
                },
                _token => {
                    if event.is_writable() {
                        println!("is_writable")
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
