# 【Rust】标注生存期

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/explicit.html>  

## 示例

生存期的标注，使用撇号来标注。

### main.rs

```rust
// 标注了两个生存期参数
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn main() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
}
```

## 总结

了解了 Rust 中的生存期的标记。

## 附录
