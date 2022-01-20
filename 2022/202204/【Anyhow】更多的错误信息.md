# 【Anyhow】更多的错误信息

## 环境

- Time 2022-01-20
- Rust 1.58.0
- Anyhow 1.0.52

## 概念

参考：<https://docs.rs/anyhow/latest/anyhow/>  

## 示例

### toml

```toml
[package]
edition = "2021"
name = "game"
version = "0.1.0"

[dependencies]
anyhow = "*"
serde = {version = "*", features = ["derive"]}
serde_json = "*"
```

### main.rs

```rust
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
}

fn main() -> Result<()> {
    let path = "user.json";
    let config = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read from {path}"))?;
    let user: User = serde_json::from_str(&config)?;
    println!("name: {}, age: {}", user.name, user.age);
    Ok(())
}
```

## 总结

对错误添加多的上下文信息，更友好和详细的错误显示。

## 附录
