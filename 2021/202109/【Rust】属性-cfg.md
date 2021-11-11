# 【Rust】属性-cfg

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/attribute/cfg.html>  

## 示例

### 属性配置

```rust
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// target_os 是 rust 自动传递的
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();
}
```

### 宏配置

target_os 由 rust 自动传入。

```rust
fn main() {
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```

## 总结

了解了 Rust 中怎么使用属性来定义库的名称和类型（在没有使用 cargo 的情况下）。

## 附录
