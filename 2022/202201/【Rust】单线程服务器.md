# 【Rust】单线程服务器

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/book/ch20-01-single-threaded.html>  

## 示例

### main.rs

```rust
use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("bind");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let length = stream.read(&mut buffer).unwrap();

    if length == 0 {
        println!("EOF");
    }

    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

### hello.html

```html
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="utf-8">
    <title>Hello!</title>
</head>

<body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
</body>

</html>
```

## 总结

使用 rust 实现了一个单线程的 web 服务器。

## 附录
