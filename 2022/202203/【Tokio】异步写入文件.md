# 【Tokio】异步写入文件

## 环境

- Time 2022-01-12
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/fs/struct.File.html>  

## 示例

### main.rs

```rust
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut file = File::create("test.txt").await.unwrap();
    file.write_all(b"jiangbo").await.unwrap();
    println!("写入完成");
}
```

### 缓存写入

```rust
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

#[tokio::main]
async fn main() {
    let file = File::create("test.txt").await.unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(b"jiangbo").await.unwrap();
    writer.flush().await.unwrap();
    println!("写入完成");
}
```

### 追加

```rust
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let file = File::create("test.txt").await?;
    let mut writer = BufWriter::new(file);
    writer.write_all(b"jiangbo").await?;
    writer.flush().await?;
    println!("写入完成");

    let file = OpenOptions::new().append(true).open("test.txt").await?;
    let mut writer = BufWriter::new(file);
    writer.write_all("\n测试追加".as_bytes()).await?;
    writer.flush().await?;
    println!("追加完成");

    Ok(())
}
```

## 总结

Tokio 对于写入文件，也可以使用异步的方式。

## 附录
