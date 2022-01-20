# 【Anyhow】轻松处理错误

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
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
}

fn main() -> Result<()> {
    let config = std::fs::read_to_string("user.json")?;
    let user: User = serde_json::from_str(&config)?;
    println!("name: {}, age: {}", user.name, user.age);
    Ok(())
}
```

### json

```json
{
    "name": "JiangBo",
    "age": 44
}
```

## 总结

使用 Anyhow 来处理不同类型的错误，可以很方便地使用问号操作符进行返回。

## 附录
