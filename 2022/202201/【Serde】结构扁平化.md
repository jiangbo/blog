# 【Serde】结构扁平化

## 环境

- Time 2021-12-04
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/attr-flatten.html>  

## 示例

### 组合公共属性

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paging {
    page_number: usize,
    page_size: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    name: String,
    paging: Paging,
}

fn main() {
    let request = Request {
        name: "JiangBo".to_owned(),
        paging: Paging {
            page_number: 4,
            page_size: 44,
        },
    };

    let json = serde_json::to_string(&request).unwrap();
    println!("{}", json);
    let req: Request = serde_json::from_str(&json).unwrap();
    println!("{:?}", req);
}
```

### 捕获剩余字段

```rust
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    name: String,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

fn main() {
    let json = r#"{
        "name": "JiangBo",
        "page_number": 4,
        "page_size": 44
      }"#;

    let req: Request = serde_json::from_str(json).unwrap();
    println!("{:?}", req);
    println!("{}", serde_json::to_string(&req).unwrap());
}
```

## 总结

使用 serde 将多层的结构体展开成一层的结构，也可以将没有定义的字段，全部放到扩展字段中。

## 附录
