# 【Rust】泛型约束

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/generics/bounds.html>  

## 示例

泛型约束，也叫有界泛型，可以限制泛型的类型。

### 约束类型

```rust
use std::fmt::Display;

// 定义一个泛型函数，要求泛型必须实现 Display
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

fn main() {
    // 编译错误，vector并没有实现 Display
    // let s = printer(vec![1]);
}
```

### 使用约束的方法

```rust
trait HasArea {
    fn area(&self) -> f64;
}

struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}
// 泛型必须实现 HasArea
fn area<T: HasArea>(t: &T) -> f64 {
    // 调用泛型约束的方法
    t.area()
}

fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    println!("Area: {}", area(&rectangle));
}
```

## 总结

了解了 Rust 中的泛型约束，也是有界泛型，和其它语言中的面向接口编程类似。

## 附录
