# 【Rust】表达式

## 环境

- Rust 1.56.1
- VSCode 1.60.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/expression.html>  

## 示例

### 语句

Rust 程序的大部分都是由语句构成的。

```rust
fn main() {
    // statement
    // statement
    // statement
}
```

### 表达式

Rust 由多种语句，最常见的是变量绑定和带分号的表达式。

```rust
fn main() {
    // 变量绑定
    let x = 5;

    // 表达式+分号;
    x;
    x + 1;
    15;
}
```

### 代码块表达式

代码块也是表达式，所以它们可以在赋值中使用。代码块的最后一条表达式将赋值给位置表达式。需要注意的是，如果代码块最后一条表达式结尾处有分号，那么返回值将变成 ()。

```rust
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 最终的结果将赋值给 y
        x_cube + x_squared + x
    };

    let z = {
        // 由于有分号，所以将单元类型赋值给 z
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
```

## 总结

了解了 Rust 中的表达式和语句，需要注意代码块的最后一个表达式。

## 附录
