# 【Rust】原始类型-整型

## 环境

- Rust 1.54.0
- VSCode 1.59.1

## 概念

Rust 中的整型可以分为有符号的整型和无符号的整型，有如下的类型：

| 长度    | 有符号  | 无符号  |
| ------- | ------- | ------- |
| 8-bit   | `i8`    | `u8`    |
| 16-bit  | `i16`   | `u16`   |
| 32-bit  | `i32`   | `u32`   |
| 64-bit  | `i64`   | `u64`   |
| 128-bit | `i128`  | `u128`  |
| arch    | `isize` | `usize` |

`isize` 和 `usize` 和具体的 CPU 架构相关。

## 示例

### 类型申明

如果没有为整型指明变量类型，默认为 `i32` 类型。  
类型声明可以在变量名的后面，也可以在具体值得后面。

```rust
fn main() {
    let x = 0;
    let y: u8 = 18;
    let z = 8u16;
    println!("x = {}, y = {}, z = {}", x, y, z);
}
```

### 下划线分割

使用下划线分割可以提高可读性，不会影响具体的值。

```rust
fn main() {
    let x = 1_0000_0000;
    println!("x = {}", x);
}
```

### 进制表示法

```rust
fn main() {
    let a = 0x10; //十六进制
    let b = 10; // 十进制
    let c = 0o10; // 八进制
    let d = 0b10; // 二进制
    println!("a={}, b={}, c={}, d={}", a, b, c, d);
}
```

### 四则运算

```rust
fn main() {
    let a = 44;
    let b = 4;

    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
}
```

### 位运算

```rust
fn main() {
    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
```

## 总结

介绍了 Rust 中的整型的一些概念和操作。

## 附录
