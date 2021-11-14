# 【Rust】多种错误类型

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/multiple_error_types.html>  

## 示例

### main.rs

```rust
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap();
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    // 错误一
    println!("The first doubled is {}", double_first(empty));

    // 错误二
    println!("The first doubled is {}", double_first(strings));
}
```

## 总结

了解了 Rust 中有可能会出现多种错误，接下来有哪些解决方式。

## 附录
