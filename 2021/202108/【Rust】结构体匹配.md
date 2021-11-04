# 【Rust】结构体匹配

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring/destructure_structures.html>  

## 示例

### 结构体匹配解构

```rust
fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };
    // 匹配结构体
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // // 解构结构体，属性的顺序不重要
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // 使用 .. 可以忽略剩下的属性
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // 如果缺少属性，会有编译错误
        //Foo { y } => println!("y = {}", y),
    }
}
```

## 总结

了解了 Rust 中的结构体的匹配和解构。

## 附录
