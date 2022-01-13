# 【Tokio】设值和取值

## 环境

- Time 2022-01-13
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://tokio.rs/tokio/tutorial/hello-tokio>  

## 示例

### main.rs

```rust
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 连接服务器地址
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 设值命令
    client.set("hello", "world".into()).await?;
    // 取值命令
    let result = client.get("hello").await?;
    // 获取结果
    println!("got value from the server; result={:?}", result);
    Ok(())
}
```

## 总结

向服务器设值和取值。

## 附录
