# 【Rust】变量类型转换

## 环境

- Rust 1.56.0
- VSCode 1.60.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/types/literals.html>  

## 示例

### 类型字面量

`std::mem::size_of_val` 可以获取变量所占用的字节数。

```rust
fn main() {
    // 通过变量类型后缀指定变量的类型
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 没有变量类型后缀，通过怎么使用变量来进行推断
    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
```

### 类型推断

如果类型不能被推断出来，整型默认是 `u32`，浮点类型默认是 `f64`。

```rust
fn main() {
    let elem = 5u8;
    let mut vec = Vec::new();

    // 通过怎么使用vec推断出了vec中的类型
    vec.push(elem);
    println!("{:?}", vec);
}
```

### 变量别名

变量类型使用大驼峰表示，基本类型除外。

```rust
// 定义了 u64 的两个别名
type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;
// 如果没有上面的属性，不以大驼峰命名会收到警告

fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
```

## 总结

了解了 Rust 中的类型字面量，类型推断和类型的别名。

## 附录
