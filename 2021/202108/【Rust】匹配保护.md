# 【Rust】匹配保护

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/guard.html>  

## 示例

### 条件过滤

```rust
fn main() {
    let pair = (2, 2);
    println!("Tell me about {:?}", pair);
    match pair {
        // if 就是保护语句，可以进行条件过滤
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
```

### 匹配所有

```rust
fn main() {
    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        // 编译器不会检查条件是否覆盖完全，需要加上全匹配
        _ => println!("Fell through"),
    }
}
```

## 总结

了解了 Rust 中的匹配保护，也就是匹配的条件过滤。

## 附录
