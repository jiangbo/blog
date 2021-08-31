# 【Rust】标准库文档注释

## 环境

- Rust 1.54.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/hello/comment.html>  
文档注释（doc comments）：文档注释会对外公开，使用者一般在使用之前，会进行阅读。  

## 示例

### 增加 Rust 文档

如果没有下载文档，可以使用命令 `rustup component add rust-docs` 增加。

### 打开文档

使用命令 `rustup doc` 打开文档，除了标准库外，还有许多的学习资料文档。

![rust doc][1]

### 查看标准库文档

点击文档中间的 `extensive API documentation` 就可以打开标准库的文档。

![rust std doc][2]

## 总结

介绍了 Rust 中的 `rust-docs`，离线查看了学习文档和标准库文档。

## 附录

[1]: images/rust-doc.png
[2]: images/rust-std-doc.png
