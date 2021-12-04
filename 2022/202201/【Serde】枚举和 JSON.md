# 【Serde】枚举和 JSON

## 环境

- Time 2021-12-04
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/enum-representations.html>  

## 示例

### 外部标签

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Animal {
    Human {
        name: String,
        age: u16,
        languages: Vec<String>,
    },
}

fn main() {
    let human = Animal::Human {
        name: "JiangBo".to_owned(),
        age: 44,
        languages: vec!["Rust".to_owned(), "Java".to_owned()],
    };

    println!("{}", serde_json::to_string(&human).unwrap());
    // {"Human":{"name":"JiangBo","age":44,"languages":["Rust","Java"]}}
}
```

### 内部标签

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")] // 新增
enum Animal {
    Human {
        name: String,
        age: u16,
        languages: Vec<String>,
    },
}

fn main() {
    let human = Animal::Human {
        name: "JiangBo".to_owned(),
        age: 44,
        languages: vec!["Rust".to_owned(), "Java".to_owned()],
    };

    println!("{}", serde_json::to_string(&human).unwrap());
    // {"type":"Human","name":"JiangBo","age":44,"languages":["Rust","Java"]}
}
```

### 相邻标签

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")] // 新增
enum Animal {
    Human {
        name: String,
        age: u16,
        languages: Vec<String>,
    },
}

fn main() {
    let human = Animal::Human {
        name: "JiangBo".to_owned(),
        age: 44,
        languages: vec!["Rust".to_owned(), "Java".to_owned()],
    };

    println!("{}", serde_json::to_string(&human).unwrap());
    // {"tag":"Human","content":{"name":"JiangBo","age":44,"languages":["Rust","Java"]}}
}
```

### 无标签

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)] // 新增
enum Animal {
    Human {
        name: String,
        age: u16,
        languages: Vec<String>,
    },
}

fn main() {
    let human = Animal::Human {
        name: "JiangBo".to_owned(),
        age: 44,
        languages: vec!["Rust".to_owned(), "Java".to_owned()],
    };

    println!("{}", serde_json::to_string(&human).unwrap());
    // {"name":"JiangBo","age":44,"languages":["Rust","Java"]}
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

使用 serde 将枚举转换为各种不同的格式，实际情况可以根据需要选择。

## 附录
