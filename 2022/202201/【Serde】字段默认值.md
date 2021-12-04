# 【Serde】字段默认值

## 环境

- Time 2021-12-04
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/attr-default.html>  

## 示例

### 调用 default 函数

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    #[serde(default)]
    age: u16,
    languages: Vec<String>,
}

fn main() {
    let json = r#"{
        "name":"JiangBo",
        "languages":["Rust","Java"]
    }"#;

    println!("{:?}", serde_json::from_str::<Person>(json).unwrap());
}
```

### 默认函数

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    // 自动调用 default_name 函数
    #[serde(default = "default_name")]
    name: String,
    age: u16,
    languages: Vec<String>,
}

fn default_name() -> String {
    "JiangBo".to_owned()
}

fn main() {
    let json = r#"{
        "name":"JiangBo",
        "age": 44,
        "languages":["Rust","Java"]
    }"#;

    println!("{:?}", serde_json::from_str::<Person>(json).unwrap());
}
```

### 默认枚举值

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u16,
    #[serde(default = "Language::rust")]
    language: Language,
}

#[derive(Serialize, Deserialize, Debug)]
enum Language {
    Java,
    Rust,
}

impl Language {
    fn rust() -> Self {
        Language::Rust
    }
}

fn main() {
    let json = r#"{
        "name":"JiangBo",
        "age": 44
    }"#;

    println!("{:?}", serde_json::from_str::<Person>(json).unwrap());
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

使用 serde 反序列化时，怎么对缺失的字段设置默认值。

## 附录
