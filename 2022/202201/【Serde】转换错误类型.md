# 【Serde】转换错误类型

## 环境

- Time 2021-12-07
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/convert-error.html>  

## 示例

### main.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Resource {
    name: String,

    #[serde(with = "as_json_string")]
    policy: Policy,
}

#[derive(Serialize, Deserialize)]
struct Policy {
    effect: String,
    action: String,
    resource: String,
}

mod as_json_string {
    use serde::de::{Deserialize, DeserializeOwned, Deserializer};
    use serde::ser::{Serialize, Serializer};
    use serde_json;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        use serde::ser::Error;
        let j = serde_json::to_string(value).map_err(Error::custom)?;
        j.serialize(serializer)
    }

    // Deserialize a string from the input format, then deserialize the content
    // of that string as JSON.
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: DeserializeOwned,
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let j = String::deserialize(deserializer)?;
        serde_json::from_str(&j).map_err(Error::custom)
    }
}

fn main() {
    let resource = Resource {
        name: "test_policy".to_owned(),
        policy: Policy {
            effect: "Allow".to_owned(),
            action: "s3:ListBucket".to_owned(),
            resource: "arn:aws:s3:::example_bucket".to_owned(),
        },
    };

    let y = serde_yaml::to_string(&resource).unwrap();
    println!("{}", y);
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

使用 serde 进行序列和反序列化时的错误处理。

## 附录
