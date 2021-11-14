# 【Rust】生存期-省略

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/scope/lifetime/elision.html>  

## 示例

在大多数的情况下，编译器可以推断出生存期，所以我们可以省略生存期标注。

### main.rs

```rust
// 生存期推断
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

// 生存期推断
fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn main() {
    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}
```

## 总结

了解了 Rust 中省略生存期标注，在编译器可以推断出来的情况下，可以直接省略。

## 附录
