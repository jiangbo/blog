# 【Rust】生存期-强转

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/scope/lifetime/lifetime_coercion.html>  

## 示例

### main.rs

```rust
// Rust 自动推导了生存期，两个参数都被申明为这个周期
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// ’a:'b表示 a 的生存期不比 'b 短
// 返回了一个较短的生存期
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Longer lifetime
    {
        let second = 3; // Shorter lifetime

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}
```

## 总结

了解了 Rust 中的强转的生存期参数的标注。

## 附录
