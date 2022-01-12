# 【Tokio】标准输入输出

## 环境

- Time 2022-01-12
- Rust 1.57.0
- Tokio 1.15.0

## 概念

参考：<https://docs.rs/tokio/latest/tokio/io/struct.Stdout.html>  

## 示例

### main.rs

```rust
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin());
    let mut buf = String::new();
    reader.read_line(&mut buf).await?;

    let mut writer = BufWriter::new(io::stdout());
    writer.write_all(buf.trim_end().as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}
```

## 总结

Tokio 对于标准输入和输出，也提供了异步的方式。

## 附录
