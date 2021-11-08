# 【Rust】闭包-输入函数

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_functions.html>  

既然闭包可以作为函数的输入参数，那么函数是否可以做为输入参数呢？可以的。

## 示例

### 函数作为参数

```rust
fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(function);
}
```

## 总结

了解了 Rust 中将函数作为另一个函数的输入参数。

## 附录
