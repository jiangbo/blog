# 【Rust】闭包

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/fn/closures.html>  

闭包，类似其它语言中的 lambda 函数。

## 示例

### 函数实现自增

```rust
fn main() {
    fn add_one(i: i32) -> i32 {
        i + 1
    }
    println!("add one: {}", add_one(1));
}
```

### 闭包实现自增

声明时使用 || 替代 () 将输入参数括起来。

```rust
fn main() {
    let add_one = |i: i32| -> i32 { i + 1 };
    println!("add one: {}", add_one(1));
}
```

### 闭包自动推导

函数参数的类型可以省略，如果函数体只有一行，大括号也可以省略。

```rust
fn main() {
    let add_one = |i| i + 1;
    println!("add one: {}", add_one(1));
}
```

### 变量捕获

可以捕获外部的变量。

```rust
fn main() {
    let n = 3;
    let add_n = |i| i + n;
    println!("add n: {}", add_n(1));
}
```

## 总结

了解了 Rust 中的闭包函数，和其它语言中的 lambda 函数类似。

## 附录
