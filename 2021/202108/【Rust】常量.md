# 【Rust】常量

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/custom_types/constants.html>

Rust 有两种常量，可以在任意作用域声明，包括全局作用域。它们都需要显式的类型声明：

- `const`: 不可改变的值（通常使用这种）。
- `static`: 可以改变的带有 `'static` 生命周期的变量，静态生命周期是推断出来的，不必指定。访问或修改可变静态变量是 `unsafe`。

## 示例

```rust
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
}
```

## 总结

了解了 Rust 中的常量类型，一个不可变的 `const` 和一个可变的 `static`。

## 附录
