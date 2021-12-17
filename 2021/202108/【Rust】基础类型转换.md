# 【Rust】基础类型转换

## 环境

- Rust 1.56.0
- VSCode 1.60.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/types/cast.html>  

Rust 没有提供基础类型间的隐式转换，可以使用 `as` 关键字进行显式的类型转换。  

## 示例

### 不支持隐式转换

```rust
fn main() {
    let decimal = 65.4321_f32;
    // 错误，不能隐式转换类型
    // let integer: u8 = decimal;
}
```

### 显式转换

```rust
fn main() {
    let decimal = 65.4321_f32;

    // 显示转换
    let integer = decimal as u8;
    let character = integer as char;
    println!("{}", integer);
    println!("{}", character);
}
```

### 浮点类型不能直接转 char

```rust
fn main() {
    let decimal = 65.4321_f32;

    // 错误，浮点类型不能转成字符类型
    // let character = decimal as char;
}
```

### 数值溢出

转换的过程中，可能会发生溢出，默认会出错，可以使用 `#![allow(overflowing_literals)]` 来抑制。

```rust
#![allow(overflowing_literals)]
fn main() {
    println!("1000 as a u8 is : {}", 1000 as u8);
    println!(" 128 as a i8 is : {}", 128 as i8);
}
```

### 浮点数转整型

```rust
fn main() {
    // 数值过大，直接转为最大值 255
    println!("300.0 is {}", 300.0_f32 as u8);
    // 数值过小，直接转为 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // NAN 直接转为 0
    println!("nan as u8 is {}", f32::NAN as u8);
}
```

### 不安全转换

如果不想使用默认的浮点到整型的转换，可以使用 unsafe 操作。

```rust
fn main() {
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
```

## 总结

了解了 Rust 中的基础类型的转换，有些可以直接转换，有些可能会导致溢出或者类型不兼容的问题。

## 附录
