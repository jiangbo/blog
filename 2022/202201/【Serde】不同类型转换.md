# 【Serde】不同类型转换

## 环境

- Time 2021-12-06
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/attr-bound.html>  

## 示例

如果类型不同，会出现错误：`"invalid type: string xxxx , expected xxxx"`,可以通过下面的方式来解决。如果需要支持多种类型的转换，参考下一篇。

### main.rs

```rust
use std::{fmt::Display, str::FromStr};

use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    #[serde(deserialize_with = "from_str")]
    age: u16,
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

fn main() {
    let json = r#"{
        "name":"JiangBo",
        "age": "44"
    }"#;

    println!("{:?}", serde_json::from_str::<Person>(json).unwrap());
}
```

## 总结

使用 serde 反序列化时，如果 JSON 中的类型和程序中的类型不匹配，可以针对某个字段进行自定义转换。

## 附录
