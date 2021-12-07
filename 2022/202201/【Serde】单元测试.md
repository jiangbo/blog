# 【Serde】单元测试

## 环境

- Time 2021-12-07
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/unit-testing.html>  

## 示例

### main.rs

```rust
use std::collections::HashMap;

use serde_test::{assert_tokens, Token};

#[test]
fn test_ser_de_empty() {
    let map = HashMap::<char, u32>::new();

    assert_tokens(&map, &[Token::Map { len: Some(0) }, Token::MapEnd]);
}

#[test]
fn test_ser_de() {
    let mut map = HashMap::new();
    map.insert('b', 20);
    map.insert('a', 10);
    map.insert('c', 30);

    assert_tokens(
        &map,
        &[
            Token::Map { len: Some(3) },
            Token::Char('b'),
            Token::I32(20),
            Token::Char('a'),
            Token::I32(10),
            Token::Char('c'),
            Token::I32(30),
            Token::MapEnd,
        ],
    );
}

fn main() {}
```

## 总结

使用 serde 进行序列化和反序列化时，怎么对 serde 进行单元测试。

## 附录
