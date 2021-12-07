# 【Serde】自定义类型转换

## 环境

- Time 2021-12-06
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/deserialize-map.html>  

## 示例

### main.rs

```rust
use std::fmt;

use serde::{de::Visitor, Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
    languages: MyVec,
}

#[derive(Debug)]
struct MyVec(Vec<String>);

struct MyVecVisitor;

impl<'de> Visitor<'de> for MyVecVisitor {
    type Value = MyVec;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("my vec error")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let vec = v.split(',').map(str::to_string).collect();
        Ok(MyVec(vec))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }
}

impl<'de> Deserialize<'de> for MyVec {
    fn deserialize<D>(deserializer: D) -> Result<MyVec, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MyVecVisitor)
    }
}

fn main() {
    let json = r#"{
        "name":"JiangBo",
        "languages": "Java,Rust"
    }"#;

    let person: Person = serde_json::from_str(json).unwrap();
    println!("{:?}", person);
}
```

## 总结

使用 serde 反序列化时，提供了自定义的类型，并且提供了自定义类型的反序列化逻辑。

## 附录
