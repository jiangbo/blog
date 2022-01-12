# 【Tokio】异步读取文件

## 环境

- Time 2022-01-12
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/fs/struct.File.html>  

## 示例

### 读取一行

```rust
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let file = File::open("test.txt").await?;
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();
    reader.read_line(&mut buffer).await?;

    println!("{}", buffer);
    Ok(())
}
```

### 循环读取行

```rust
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let file = File::open("test.txt").await?;
    let mut lines = BufReader::new(file).lines();

    while let Some(line) = lines.next_line().await? {
        println!("{}", line);
    }
    Ok(())
}
```

### 全部读取

```rust
use tokio::fs::read_to_string;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let content = read_to_string("test.txt").await?;
    println!("{}", content);
    Ok(())
}
```

## 总结

Tokio 对于读取文件，也可以使用异步的方式。

## 附录
