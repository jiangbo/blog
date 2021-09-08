# 【Rust】原始类型-布尔

## 环境

- Rust 1.54.0
- VSCode 1.59.1

## 概念

布尔类型有两种， `true` 和 `false`。

## 示例

### 类型申明

```rust
fn main() {
    let x = false;
    let y: bool = true;
    println!("x = {}, y = {}", x, y);
}
```

### 布尔运算

```rust
fn main() {
    let a = false | true;
    let b: bool = true & false;
    let c = false || true;
    let d: bool = true && false;
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
}
```

## 总结

介绍了 Rust 中的布尔类型和运算。

## 附录
