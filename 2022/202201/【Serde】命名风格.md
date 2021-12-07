# 【Serde】命名风格

## 环境

- Time 2021-12-07
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/container-attrs.html>  

## 示例

- `lowercase` 全小写
- `UPPERCASE` 全大写
- `PascalCase` 大驼峰
- `camelCase` 小驼峰
- `snake_case` 小写下划线
- `SCREAMING_SNAKE_CASE` 大写下划线
- `kebab-case` 小写中划线
- `SCREAMING-KEBAB-CASE` 大写中划线

### main.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Person {
    person_name: String,
    person_age: u16,
}

fn main() {
    let person = Person {
        person_name: "JiangBo".to_owned(),
        person_age: 44,
    };

    let json = serde_json::to_string(&person).unwrap();
    println!("{}", json);

    let p: Person = serde_json::from_str(&json).unwrap();
    println!("{:?}", p);
}
```

## 总结

使用 serde 进行序列化和反序列化时，可以设置不同的命名风格。

## 附录
