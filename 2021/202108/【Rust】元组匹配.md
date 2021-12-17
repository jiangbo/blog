# 【Rust】元组匹配

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring/destructure_tuple.html>  

## 示例

### 元组匹配解构

```rust
fn main() {
    let triple = (0, -2, 3);
    match triple {
        // 解构第二和第三个值
        (0, y, z) => println!("0 - {:?} - {:?}", y, z),
        // 只匹配第一个值，后面的没有匹配和解构，.. 可以忽略元组剩下的值
        (1, ..) => println!("First is `1`"),
        _ => println!("It doesn't matter what they are"),
        // `_` 表示任何值
    }
}
```

## 总结

了解了 Rust 中的 match 关键字，可以用来匹配元组，匹配项可以使用使用解构。

## 附录
