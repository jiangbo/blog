# 【Serde】JSON 转结构体

## 环境

- Time 2021-12-02
- VSCode 1.61.2
- Rust 1.56.1
- Serde 1.0.72

## 概念

参考：<https://docs.serde.rs/serde_json/index.html#parsing-json-as-strongly-typed-data-structures>  

## 示例

### main.rs

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: u16,
    languages: Vec<String>,
}

fn main() {
    let data = r#"
    {
        "name": "JiangBo",
        "age": 44,
        "languages": [
            "Rust",
            "Java"
        ]
    }"#;

    let person: Person = serde_json::from_str(data).unwrap();
    println!(
        "{}, {} years old, like {}",
        person.name, person.age, person.languages[0]
    );
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

使用 serde 将一个字符串转为结构体。

## 附录
