# 【Rust】行注释

## 环境

- Rust 1.54.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/hello/comment.html>  
行注释（line comments）：当代码逻辑很复杂，需要额外说明时，可以使用行注释来帮助人们理解。

> 行注释会直接被编译器忽略，注释的有效范围只有一行。

## 示例

### 单行注释

```rust
fn main() {
    // 打印 hello world
    println!("Hello, world!");
}
```

### 多行注释

因为行注释的有效范围只有一行，如果有多行需要注释，则每行前面都要加 `//`。

```rust
// 第一行注释
// 第二行注释
// 第三行注释
```

### 行尾注释

```rust
fn main() {
    println!("Hello, world!"); // 打印 hello world
}
```

## 总结

介绍了 Rust 中的行注释的使用方式，以及常用的注释结构。

## 附录
