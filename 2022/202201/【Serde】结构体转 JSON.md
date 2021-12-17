# 【Serde】结构体转 JSON

## 环境

- Time 2021-12-02
- VSCode 1.61.2
- Rust 1.56.1

## 概念

参考：<https://docs.serde.rs/serde_json/index.html#serde-json>  

## 示例

### main.rs

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u16,
    languages: Vec<String>,
}

fn main() {
    let person = Person {
        name: "JiangBo".to_owned(),
        age: 44,
        languages: vec!["Rust".to_owned(), "Java".to_owned()],
    };

    let json = serde_json::to_string(&person).unwrap();
    println!("{}", json);
}
```

### Cargo.toml

```toml
[package]
edition = "2021"
name = "game"
version = "0.1.0"

[dependencies]
serde = {version = "1.0.130", features = ["derive"]}
serde_json = "1.0.72"
```

## 总结

使用 serde 将一个对象转为 JSON。

## 附录
