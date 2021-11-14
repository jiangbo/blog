# 【Rust】迭代中错误一

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/iter_result.html>  

## 示例

### 包含错误值

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);
}
```

### 过滤错误值

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);
}
```

## 总结

了解了 Rust 中，迭代的过程中，可以将发生错误的值过滤掉。

## 附录
