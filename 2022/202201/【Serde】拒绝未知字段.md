# 【Serde】拒绝未知字段

## 环境

- Time 2021-12-07
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/container-attrs.html>  

## 示例

### main.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Person {
    name: String,
}

fn main() {
    let json = r#"
        {
            "name": "JiangBo",
            "age": 44
        }"#;

    // 报错 "unknown field `xxxx`, expected `yyyy`"
    let p: Person = serde_json::from_str(json).unwrap();
    println!("{:?}", p);
}
```

## 总结

使用 serde 进行反序列化时，可以设置拒绝未知的字段。

## 附录
