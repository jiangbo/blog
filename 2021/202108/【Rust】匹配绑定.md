# 【Rust】匹配绑定

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/binding.html>  

## 示例

### 绑定到变量

```rust
fn age() -> u32 {
    15
}
fn main() {
    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // 如果要使用变量，则需要绑定到变量
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}
```

### 绑定到枚举

```rust
fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    match some_number() {
        // 枚举的解构绑定
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}
```

## 总结

了解了 Rust 中的匹配绑定，在匹配的过程中，如果没有直接变量，可以使用变量绑定。

## 附录
