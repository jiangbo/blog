# 【Rust】错误-恐慌

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/error/panic.html>  

## 示例

恐慌（panic）和其它语言中的异常和错误类似，一般针对不可处理的错误。如果没有经过处理，就会直接退出程序。

### main.rs

```rust
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }
    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
}
```

## 总结

了解了 Rust 中 panic 的使用方式。

## 附录
