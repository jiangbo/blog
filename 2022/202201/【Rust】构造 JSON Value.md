# 【Rust】构造 JSON Value

## 环境

- Time 2021-12-02
- VSCode 1.61.2
- Rust 1.56.1
- Serde 1.0.72

## 概念

参考：<https://docs.serde.rs/serde_json/index.html#serde-json>  

## 示例

可以在 JSON 中写变量。

### main.rs

```rust
use serde_json::json;

fn main() {
    let name = "JiangBo";
    let rust = "Rust";
    let value = json!(
    {
        "name": name,
        "age": 44,
        "languages": [
            rust,
            "Java"
        ]
    });

    println!(
        "{}, {} years old, like {}",
        value["name"], value["age"], value["languages"][0]
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

使用 serde 将一个字符串转为 Value 对象，并且从里面获取了想要的值。

## 附录
