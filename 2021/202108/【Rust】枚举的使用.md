# 【Rust】枚举的使用

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum/enum_use.html>

枚举类型的简单使用。

## 示例

### 类型别名

Self 就是实现当前 trait 的类型的别名。

```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
    let x = Operations::Add;
    match x {
        Operations::Add => println!("add..."),
        Operations::Subtract => println!("subtract..."),
    }

    println!("subtract: {}", Operations::Subtract.run(1, 1));
}
```

### 使用 use 导入

使用 use 声明的话，就可以不写出名称的完整路径。

```rust
#![allow(dead_code)]
enum Status {
    Rich,
    Poor,
}

fn main() {
    use Status::{Poor, Rich};

    let status = Poor;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }
}
```

### C 类型枚举

```rust
#![allow(dead_code)]
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
```

## 总结

使用了 Rust 中的枚举类型。

## 附录
