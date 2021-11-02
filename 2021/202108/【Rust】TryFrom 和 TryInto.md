# 【Rust】TryFrom 和 TryInto

## 环境

- Rust 1.56.0
- VSCode 1.60.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/conversion/try_from_try_into.html>  

TryFrom/TryInto 和 From/Into 类似，不过用于可能发生异常的转换，所以返回结果是 Result。

## 示例

### TryFrom

```rust
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
    type Error = ();
}

fn main() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
}
```

### TryInto

```rust
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
    type Error = ();
}

fn main() {
    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
```

## 总结

了解了 Rust 中的 TryFrom 和 TryInto trait，用来进行可能发生异常的自定义类型转换。

## 附录
