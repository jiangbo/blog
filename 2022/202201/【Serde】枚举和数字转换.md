# 【Serde】枚举和数字转换

## 环境

- Time 2021-12-06
- Rust 1.57.0
- serde 1.0.130
- serde_json 1.0.72

## 概念

参考：<https://serde.rs/enum-number.html>  

## 示例

### main.rs

```rust
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum SmallPrime {
    Two = 2,
    Three = 3,
    Five = 5,
    Seven = 7,
}

fn main() {
    use SmallPrime::*;
    let nums = vec![Two, Three, Five, Seven];

    println!("{}", serde_json::to_string(&nums).unwrap());
    assert_eq!(Two, serde_json::from_str("2").unwrap());
}
```

## 总结

使用 serde 时，可以将枚举映射为数字，也可以将数字映射为枚举。

## 附录
