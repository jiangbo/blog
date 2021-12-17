# 【Rust】路径

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std_misc/path.html>  

## 示例

### main.rs

```rust
use std::path::Path;

fn main() {
    let path = Path::new(".");
    let _display = path.display();
    let new_path = path.join("a").join("b");

    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
```

## 总结

了解了 Rust 中路径的使用，windows 和 linux 系统的路径符号不同。

## 附录
