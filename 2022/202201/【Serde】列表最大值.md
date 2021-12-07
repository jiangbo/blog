# 【Serde】列表最大值

## 环境

- Time 2021-12-06
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/stream-array.html>  

## 示例

### main.rs

```rust
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};

use std::marker::PhantomData;
use std::{cmp, fmt};

#[derive(Deserialize)]
struct Outer {
    #[serde(deserialize_with = "deserialize_max")]
    max_value: u64,
}

fn deserialize_max<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Ord,
    D: Deserializer<'de>,
{
    struct MaxVisitor<T>(PhantomData<T>);
    impl<'de, T> Visitor<'de> for MaxVisitor<T>
    where
        T: Deserialize<'de> + Ord,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a nonempty sequence of numbers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<T, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut max = seq
                .next_element()?
                .ok_or_else(|| de::Error::custom("no values"))?;

            while let Some(value) = seq.next_element()? {
                max = cmp::max(max, value);
            }
            Ok(max)
        }
    }

    let visitor = MaxVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}

fn main() {
    let j = r#"
        {
          "max_value": [
            256,
            100,
            384,
            314,
            271
          ]
        }
    "#;

    let out: Outer = serde_json::from_str(j).unwrap();

    // Prints "max value: 384"
    println!("max value: {}", out.max_value);
}
```

## 总结

使用 serde 反序列化时，从一个列表中找到最大值进行存储。

## 附录
