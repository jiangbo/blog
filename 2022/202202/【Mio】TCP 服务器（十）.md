# 【Mio】TCP 服务器（十）

## 环境

- Time 2022-01-08
- Rust 1.57.0
- mio 0.8

## 概念

参考：<https://github.com/tokio-rs/mio/blob/master/examples/tcp_server.rs>  

处理读取时数据超长问题。

> TCP 服务器基本功能实现完成，只能做练习使用，不可用于生产环境。

## 示例

### 扩容

```rust
loop {
    match client.read(&mut buffer) {
        Ok(0) => return Ok(true),
        Ok(n) => {
            bytes_read += n;
            if bytes_read == buffer.len() {
                buffer.resize(buffer.len() + 1024, 0);
            }
        }
        Err(e) if e.kind() == WouldBlock => break,
        Err(err) => return Err(err),
    }
}
```

## 总结

检查客户端数据是否超过了缓存大小，超过了就进行扩容。

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
    io::{self, ErrorKind::WouldBlock, Read, Write},
    str::from_utf8,
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
                        if handle_client(event, client)? {
                            if let Some(mut client) = clients.remove(&token) {
                                println!("{} 连接已关闭", client.peer_addr()?);
                                poll.registry().deregister(&mut client)?;
                            }
                        }
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

fn handle_client(event: &Event, client: &mut TcpStream) -> io::Result<bool> {
    if event.is_readable() {
        let mut buffer = vec![0; 4096];
        let mut bytes_read = 0;
        loop {
            match client.read(&mut buffer) {
                Ok(0) => return Ok(true),
                Ok(n) => {
                    bytes_read += n;
                    if bytes_read == buffer.len() {
                        buffer.resize(buffer.len() + 1024, 0);
                    }
                }
                Err(e) if e.kind() == WouldBlock => break,
                Err(err) => return Err(err),
            }
        }

        if bytes_read != 0 {
            let received = &buffer[..bytes_read];
            let str = from_utf8(received).unwrap();
            println!("收到数据：{}", str);
            client.write_all(str.to_ascii_uppercase().as_bytes())?;
        }
    };
    Ok(false)
}
```
