# 【Rust】Result-提前返回

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/result/early_returns.html>  

## 示例

### main.rs

```rust
use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    let first_number = match first.parse::<i32>() {
        Ok(first_number) => first_number,
        // 提前返回
        Err(e) => return Err(e),
    };

    let second_number = match second.parse::<i32>() {
        Ok(second_number) => second_number,
        // 提前返回
        Err(e) => return Err(e),
    };

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

了解了 Rust 中，Result 如果发生错误，怎么进行提前返回。

## 附录
