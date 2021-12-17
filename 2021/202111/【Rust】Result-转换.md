# 【Rust】Result-转换

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/result/result_map.html>  

## 示例

Result 的 map 方法可以对值进行转换。

### 繁琐版本

```rust
use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    match first.parse::<i32>() {
        Ok(f1) => match second.parse::<i32>() {
            Ok(s2) => Ok(f1 * s2),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);
}
```

### 简单版本

比上面的要简单一点。

```rust
use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    first.parse::<i32>().and_then(|first_number| {
        second
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);
}
```

## 总结

了解了 Rust 中 Result 类型使用 map 和 and_then 方法。

## 附录
