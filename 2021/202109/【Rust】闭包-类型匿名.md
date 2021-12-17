# 【Rust】闭包-类型匿名

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/fn/closures/anonymity.html>  

当闭包被定义的时候，编译器会创建一个匿名的结构体来存储捕获的变量。同时通过实现 Fn、FnMut 和 FnOnce 其中之一的 trait 来实现功能。然后将匿名的结构体赋值给变量。

## 示例

### Fn

```rust
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let x = 7;
    let print = || println!("{}", x);
    apply(print);
}
```

## 总结

了解了 Rust 中闭包的实现方式，定义一个匿名的结构体存储变量，并实现三种 Fn 中的一个。

## 附录
