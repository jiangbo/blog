# 【Rust】块注释

## 环境

- Rust 1.54.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/hello/comment.html>  
块注释（block comments）：当代码逻辑很复杂，需要额外说明时，可以使用块注释来帮助人们理解。

> 块注释会直接被编译器忽略，注释的有效范围可以多行。

## 示例

### 块注释

```rust
fn main() {
    /* 打印 hello world */
    println!("Hello, world!");
}
```

### 多行注释1

```rust
fn main() {
    /*
     * 第一行注释
     * 第二行注释
     * 第三行注释
     */
    println!("Hello, world!");
}
```

### 多行注释2

这个和之前的相比，没有实际区别，只是之前的更美观，推荐使用前面的方式。

```rust
fn main() {
    /* 第一行注释
    第二行注释
    第三行注释 */
    println!("Hello, world!");
}
```

### 行尾注释

```rust
fn main() {
    println!("Hello, world!"); /* 打印 hello world */
}
```

### 行内注释

```rust
fn main() {
    let x = 1 + 2 /*+ 3*/ + 4 + 5;
    println!("{}", x);
}
```

### 嵌套注释

块注释可以嵌套，但是开始和结束的个数要相匹配。

```rust
fn main() {
    /* let x = 1 + 2 /*+ 3*/ + 4 + 5; */
    let x = 1;
    println!("{}", x);
}
```

## 总结

介绍了 Rust 中的块注释的使用方式，以及常用的注释结构。

## 附录
