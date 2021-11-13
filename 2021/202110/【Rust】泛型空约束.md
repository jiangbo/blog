# 【Rust】泛型空约束

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/generics/bounds/testcase_empty.html>  

## 示例

泛型空约束，trait 实现中可以不包含任何内容，只把它当做一种约束。

### main.rs

```rust
struct Cardinal;
struct BlueJay;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 定义的函数只约束了类型，其实现中根本不限制具体是什么类型
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // 编译错误，blue_jay 没有实现 red
    // println!("A turkey is {}", red(&blue_jay));
}
```

## 总结

了解了 Rust 中的空约束，可以用来限制传入的类型，其实现可以不关心具体类型，甚至可以不使用。

## 附录
