# 【Rust】Result-别名

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/result/result_alias.html>  

## 示例

### main.rs

```rust
use std::num::ParseIntError;

// 别名
type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first: &str, second: &str) -> AliasedResult<i32> {
    first.parse::<i32>().and_then(|first_number| {
        second
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: AliasedResult<i32>) {
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

了解了 Rust 中给 Result 取别名。

## 附录
