# 【Rust】格式化输出-debug

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

Debug 输出使用 `{:?}` 来进行打印，所有的标准库中的类型，都可以使用 debug 来输出。  
Debug 主要是面向程序的输出，一般来说，使用 `derive` 来自动实现 `Debug`，使用 `{:#?}` 来进行美化打印。  
如果要实现 debug 输出，需要实现 `std::fmt::Debug` 这个 trait。

> trait 可以先简单理解为其它编程语言中的接口

## 示例

### debug 输出

```rust
fn main() {
    let a = "name";
    let b = 44;

    println!("{:?}, {:?}", a, b);
}
```

### 元组输出

```rust
fn main() {
    let a: [i32; 4] = [44; 4];
    println!("{:?}", a);
}
```

### 自动实现

```rust
fn main() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    println!("{:?}", origin);
}
```

### 美化输出

```rust
fn main() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    println!("{:#?}", origin);
}
```

## 总结

介绍了使用 debug 进行格式化输出。

## 附录
