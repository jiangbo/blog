# 【Rust】From 和 Into

## 环境

- Rust 1.56.0
- VSCode 1.60.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/conversion/from_into.html>  

基础类型可以通过 `as` 关键字进行转换，如果是自定义类型，则通过 From 和 Into 进行转换。
这两个是 trait，可以理解为其它编程语言中的接口，他们是相关联的。

## 示例

### From

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}
```

### Into

如果实现了 From，则自动获得了 Into，不过需要手动指明类型。

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
```

## 总结

了解了 Rust 中的 From 和 Into trait，用来进行自定义类型转换。

## 附录
