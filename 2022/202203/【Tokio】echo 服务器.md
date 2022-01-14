# 【Tokio】echo 服务器

## 环境

- Time 2022-01-13
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://tokio.rs/tokio/tutorial/io>  

## 示例

### main.rs

```rust
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut socket, address) = listener.accept().await?;
        println!("客户端: {}", address);
        // 提交任务
        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            if tokio::io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}
```

## 总结

实现了一个 echo 服务器。

## 附录
