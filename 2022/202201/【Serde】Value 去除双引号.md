# 【Serde】Value 去除双引号

## 环境

- Time 2021-12-02
- VSCode 1.61.2
- Rust 1.56.1
- Serde 1.0.72

## 概念

参考：<https://docs.serde.rs/serde_json/index.html#operating-on-untyped-json-values>  

## 示例

转换出来的 Value 对象，获取值的时候，有双引号，将其去掉。

### main.rs

```rust
use serde_json::Value;

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

    let value: Value = serde_json::from_str(data).unwrap();
    println!(
        "{}, {} years old, like {}",
        value["name"].as_str().unwrap(),
        value["age"],
        value["languages"][0].as_str().unwrap()
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
serde_json = "1.0.72"
```

## 总结

使用 serde 将一个字符串转为 Value 对象，在取值的时候，将其中的双引号进行了去除。

## 附录
