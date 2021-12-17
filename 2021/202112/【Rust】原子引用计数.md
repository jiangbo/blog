# 【Rust】原子引用计数

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std/arc.html>  

## 示例

### main.rs

```rust
fn main() {
    use std::sync::Arc;
    use std::thread;

    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
}
```

## 总结

了解了 Rust 中标准库中的 Arc 的使用。

## 附录
