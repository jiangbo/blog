# 【Rust】宏-重复

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/macros/repeat.html>  

## 示例

使用星号表示零到多次，使用加号表示一到多次。

### main.rs

```rust
macro_rules! find_min {
    ($x:expr) => ($x);
    // 加号表示最少出现一次
    ($x:expr, $($y:expr),+) => (
        // 递归调用
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
```

## 总结

了解了 Rust 中定义宏时，怎么可以让参数变成可以多个。

## 附录
