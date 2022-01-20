# 【Anyhow】错误向下转型

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

#[derive(Deserialize, Default)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let user = read_user();
    user.unwrap_or_else(|e| match e.downcast_ref::<std::io::Error>() {
        Some(e) => {
            println!("downcast error: {:?}", e.kind());
            User::default()
        }
        None => unreachable!(),
    });
}

fn read_user() -> Result<User> {
    let config = std::fs::read_to_string("user.json")?;
    let user: User = serde_json::from_str(&config)?;
    println!("name: {}, age: {}", user.name, user.age);
    Ok(user)
}
```

## 总结

可以对返回的错误进行向下转型，对不同的错误进行不同的处理。

## 附录
