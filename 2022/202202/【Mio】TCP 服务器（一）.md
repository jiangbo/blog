# 【mio】TCP 服务器（一）

## 环境

- Time 2022-01-07
- Rust 1.57.0
- mio 0.8

## 概念

参考：<https://github.com/tokio-rs/mio/blob/master/examples/tcp_server.rs>  

实现一个 TCP 服务器，先编写绑定端口和获取连接的客户端信息的逻辑。

## 示例

### 绑定 IP 和端口

```rust
let addr = "127.0.0.1:4444".parse().unwrap();
let mut server = TcpListener::bind(addr)?;
```

### 监听 epoll 事件

```rust
let mut poll = Poll::new()?;
let mut events = Events::with_capacity(128);
poll.registry()
    .register(&mut server, SERVER, Interest::READABLE)?;
```

### 事件循环

```rust
loop {
    poll.poll(&mut events, None)?;
    for _event in events.iter() {
        match server.accept() {
            Ok((_, address)) => {
                println!("客户端: {}", address);
            }
            Err(e) => return Err(e),
        };
    }
}
```

### 使用 nc 命令访问

```text
nc 127.0.0.1 4444
客户端: 127.0.0.1:55348
```

## 总结

使用 mio 编写 TCP 服务器，可以正确监听端口，并且获取到客户端的信息。

## 附录

### 完整代码

```rust
use mio::{net::TcpListener, Events, Interest, Poll, Token};
use std::io;

const SERVER: Token = Token(0);
fn main() -> io::Result<()> {
    let addr = "127.0.0.1:4444".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    loop {
        poll.poll(&mut events, None)?;
        for _event in events.iter() {
            match server.accept() {
                Ok((_, address)) => {
                    println!("客户端: {}", address);
                }
                Err(e) => return Err(e),
            };
        }
    }
}
```
