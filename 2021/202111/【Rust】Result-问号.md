# 【Rust】Result-问号

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/result/enter_question_mark.html>  

## 示例

如果想得到值而不产生恐慌，有一种简单的方式，那就是使用问号。

### main.rs

```rust
use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    let first_number = first.parse::<i32>()?;
    let second_number = second.parse::<i32>()?;
    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
```

## 总结

了解了 Rust 中，Result 怎么使用问号进行取值。

## 附录
