# 【Serde】重命名

## 环境

- Time 2021-12-07
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/container-attrs.html>  

## 示例

1. `#[serde(rename = "xxxx")]` 针对序列化和反序列化同名
2. `#[serde(rename(serialize = "xxxx"))]` 针对序列化
3. `#[serde(rename(deserialize = "xxxx"))]` 针对反序列化
4. `#[serde(rename(serialize = "xxxx", deserialize = "yyyy"))]` 针对不同名

其它的属性宏也有类似的命名约定。

### main.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    #[serde(rename = "my_name")]
    name: String,
    age: u16,
}

fn main() {
    let person = Person {
        name: "JiangBo".to_owned(),
        age: 44,
    };

    let json = serde_json::to_string(&person).unwrap();
    println!("{}", json);

    let p: Person = serde_json::from_str(&json).unwrap();
    println!("{:?}", p);
}
```

## 总结

使用 serde 进行序列化和反序列化时，对 JSON 中的字段进行重命名。

## 附录
