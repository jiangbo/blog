# 【mio】TCP 服务器（二）

## 环境

- Time 2022-01-07
- Rust 1.57.0
- mio 0.8

## 概念

参考：<https://github.com/tokio-rs/mio/blob/master/examples/tcp_server.rs>  

epoll 的触发方式有两种，一种是水平触发（Level Trigger，LT），一种是边缘触发（Edge Trigger，ET），关于两种模式的区别参考其它资料。在 mio 中，只支持 ET 模式，原因是不同的平台可能不兼容，具体参考 <https://github.com/tokio-rs/mio/issues/928>。

由于使用的 ET 模式，所以每次进行读取时，需要将所有的数据和事件读取完，直到 EWOULDBLOCK 为止。

## 示例

### 事件循环

```rust
loop {
    poll.poll(&mut events, None)?;
    for _event in events.iter() {
        // 可能触发了多个连接请求，处理所有的请求。
        loop {
            match server.accept() {
                Ok((_, address)) => println!("客户端: {}", address),
                // 读取
                Err(e) if e.kind() == WouldBlock => break,
                Err(e) => return Err(e),
            };
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

介绍了 mio 中的事件触发模式，以及使用循环来将所有的事件处理完成，直到 WouldBlock 为止。

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

    loop {
        poll.poll(&mut events, None)?;
        for _event in events.iter() {
            loop {
                match server.accept() {
                    Ok((_, address)) => println!("客户端: {}", address),
                    Err(e) if e.kind() == WouldBlock => break,
                    Err(e) => return Err(e),
                };
            }
        }
    }
}
```
