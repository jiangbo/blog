# 【Serde】元组和 JSON

## 环境

- Time 2021-12-04
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/json.html>  

## 示例

serde 会将元组转换成 JSON 中数组的形式。

### main.rs

```rust
fn main() {
    let tup = ("JiangBo", 44);

    let json = serde_json::to_string(&tup).unwrap();
    println!("{}", json);

    let tup: (String, u16) = serde_json::from_str(&json).unwrap();
    println!("{:?}", tup);
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

使用 serde 将一个元组转换成 JSON，再将其转换回来。

## 附录
