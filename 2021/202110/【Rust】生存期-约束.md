# 【Rust】生存期-约束

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/scope/lifetime/lifetime_bounds.html>  

## 示例

### main.rs

```rust
use std::fmt::Debug;

// 包含一个生存期参数 ‘a
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

// 生存期 + 泛型的写法
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
}
```

## 总结

了解了 Rust 中的约束的生存期参数的标注。

## 附录
