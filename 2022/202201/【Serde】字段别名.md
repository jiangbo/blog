# 【Serde】字段别名

## 环境

- Time 2021-12-07
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/field-attrs.html>  

## 示例

### main.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    #[serde(alias = "person_name")]
    name: String,
    age: u16,
}

fn main() {
    let json = r#"
        {
            "person_name": "JiangBo",
            "age": 44
        }"#;

    let p: Person = serde_json::from_str(json).unwrap();
    println!("{:?}", p);
}
```

## 总结

使用 serde 进行序列化和反序列化时，可以设置字段的别名。

## 附录
