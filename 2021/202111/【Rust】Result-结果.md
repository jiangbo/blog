# 【Rust】Result-结果

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/result.html>  

## 示例

Result 和 Option 有点类似，不过它代表可能失败，而不是可能不存在。

### 恐慌

```rust
fn multiply(first: &str, second: &str) -> i32 {
    let first_number = first.parse::<i32>().unwrap();
    let second_number = second.parse::<i32>().unwrap();
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}
```

### 在 main 中使用

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number = "10";
    let number = match number.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```

## 总结

了解了 Rust 中 Result 类型，一般是在可能发生错误的时候使用。

## 附录
