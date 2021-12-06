# 【Serde】多种类型转换

## 环境

- Time 2021-12-06
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://stackoverflow.com/questions/37870428/convert-two-types-into-a-single-type-with-serde>  

## 示例

如果在 JSON 中某个字段有多种类型，可以使用下面的方式处理。

### 直接转

```rust
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    #[serde(deserialize_with = "from_str")]
    age: u16,
}

#[derive(Deserialize)]
#[serde(untagged)] // 枚举类型的无标签方式
enum StrOrU16 {
    String(String),
    U64(u16),
}

fn from_str<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match StrOrU16::deserialize(deserializer)? {
        StrOrU16::String(v) => v.parse().unwrap_or_default(),
        StrOrU16::U64(v) => v,
    })
}

fn main() {
    // 字符串类型的 age
    let json = r#"{
        "name":"JiangBo",
        "age": "44"
    }"#;

    println!("{:?}", serde_json::from_str::<Person>(json).unwrap());

    // 数字类型的 age
    let json = r#"{
        "name":"JiangBo",
        "age": 44
    }"#;

    println!("{:?}", serde_json::from_str::<Person>(json).unwrap());
}
```

### 转为枚举

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
    age: StrOrU16,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)] // 枚举类型的无标签方式
enum StrOrU16 {
    String(String),
    U64(u16),
}

fn main() {
    let json = r#"{
        "name":"JiangBo",
        "age": "44"
    }"#;

    println!("{:?}", serde_json::from_str::<Person>(json).unwrap());

    let json = r#"{
        "name":"JiangBo",
        "age": 44
    }"#;

    println!("{:?}", serde_json::from_str::<Person>(json).unwrap());
}
```

## 总结

使用 serde 反序列化时，如果 JSON 中的类型和程序中的类型不匹配，可以使用上面的方式转换。

## 附录
