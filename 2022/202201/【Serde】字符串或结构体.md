# 【Serde】字符串或结构体

## 环境

- Time 2021-12-07
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/string-or-struct.html>  

## 示例

### main.rs

```rust
use std::collections::BTreeMap as Map;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

fn main() {
    let build_string = "
        build: ./dir
    ";
    let service: Service = serde_yaml::from_str(build_string).unwrap();
    println!("{:?}", service);

    let build_struct = "
        build:
          context: ./dir
          dockerfile: Dockerfile-alternate
          args:
            buildno: '1'
    ";
    let service: Service = serde_yaml::from_str(build_struct).unwrap();
    println!("{:?}", service);
}

#[derive(Debug, Deserialize)]
struct Service {
    #[serde(deserialize_with = "string_or_struct")]
    build: Build,
}

#[derive(Debug, Deserialize)]
struct Build {
    context: String,

    dockerfile: Option<String>,
    #[serde(default)]
    args: Map<String, String>,
}

impl FromStr for Build {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(Build {
            context: s.to_string(),
            dockerfile: None,
            args: Map::new(),
        })
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = ()>,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = ()>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
```

### Cargo.toml

```toml
[package]
edition = "2021"
name = "game"
version = "0.1.0"

[dependencies]

serde = {version = "1", features = ["derive"]}
serde-transcode = "1"
serde_json = "1"
serde_yaml = "*"
```

## 总结

使用 serde 将不同的 yaml 结构转换成了相同的结构体。

## 附录
