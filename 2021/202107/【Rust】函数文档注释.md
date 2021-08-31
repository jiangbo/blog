# 【Rust】函数文档注释

## 环境

- Rust 1.54.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/hello/comment.html>  
文档注释（doc comments）：文档注释会对外公开，使用者一般在使用之前，会进行阅读。  
文档注释包含两种，这里介绍第一种：

- `///` 为接下来的项（the following item）生成注释，一般是函数。
- `//!` 为整个项生成注释，一般是整个文件。

> 文档注释可以生成 html 查看，支持 markdown 格式。

## 示例

### 函数注释

```rust
/// 主函数
fn main() {
    let x = 1;
    println!("{}", x);
}

/// 加法函数
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

### 生成文档

可以使用命令 `cargo doc` 来生成文档注释的 html 格式，在 `target/doc` 目录下：

```txt
PS C:\Users\jiangbo\work\workspace\rust\hello_world> cargo doc
 Documenting hello_world v0.1.0 (C:\Users\jiangbo1446\work\workspace\rust\hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.79s
```

### 生成文档并打开

如果想直接查看文档，可以打开生成的文档，也可以直接使用 `cargo doc --open` 命令。

![open function doc comments][1]

## 总结

介绍了 Rust 中的函数文档注释的使用方式，生成、打开、查看文档注释。

## 附录

[1]: images/function-doc-comments.png
