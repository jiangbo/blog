# 【Rust】宏规则

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/macros.html>  

## 示例

宏和函数有点像，不过是以叹号结尾的，经常使用的 `println!` 就是一个宏。宏不是函数调用，而是直接展开到代码中。

### main.rs

```rust
// 定义一个 say_hello 的宏
macro_rules! say_hello {
    // () 表示没有参数
    () => {
        // 宏会展开成这个样子
        println!("Hello!")
    };
}

fn main() {
    // 使用宏
    say_hello!();
}
```

## 总结

了解了 Rust 中怎么定义宏和使用宏。

## 附录
