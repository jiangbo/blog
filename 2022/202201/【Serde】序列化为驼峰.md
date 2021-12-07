# 【Serde】序列化为驼峰

## 环境

- Time 2021-12-06
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/attr-rename.html>  

## 示例

### main.rs

```rust
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    first_name: String,
    last_name: String,
}

fn main() {
    let person = Person {
        first_name: "Graydon".to_string(),
        last_name: "Hoare".to_string(),
    };

    let json = serde_json::to_string_pretty(&person).unwrap();

    // Prints:
    //
    //    {
    //      "firstName": "Graydon",
    //      "lastName": "Hoare"
    //    }
    println!("{}", json);
}
```

## 总结

使用 serde 序列化时，可以使用属性宏修改命名的策略。

## 附录
