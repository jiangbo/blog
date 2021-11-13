# 【Rust】生存期

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html>  

## 示例

生存期（lifetime），也叫生命周期等，rust 使用生存期来保证所有的借用都是有效的。

### main.rs

```rust
fn main() {
    let i = 3;

    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
        // borrow1 在这里结束
    }
    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
        // borrow2 在这里结束
    }
    // i 的生存期从申明到这里结束
}
```

## 总结

了解了 Rust 中的变量的生存期。

## 附录
