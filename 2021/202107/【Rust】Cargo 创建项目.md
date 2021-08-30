# 【Rust】Cargo 创建项目

## 环境

- Windows 10
- Rust 1.54.0

## Hello World

根据传统，首先编写一个 Hello World 程序。  
参考这里的代码：<https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html>

> Cargo 是 Rust 的包管理器，和 Java 中的 Maven 类似。

### 创建项目

运行 `cargo new hello_world` 命令后，得到和 [【Rust】使用 Cargo 编译和运行][1] 中一样的目录结构。  
并且在此基础上，还生成了 Git 忽略的文件和本地仓库，这里不介绍 Git 相关基础。  

```txt
C:\Users\jiangbo\work\workspace\rust>cargo new hello_world
     Created binary (application) `hello_world` package
```

### 使用 Cargo 运行程序

```txt
C:\Users\jiangbo\work\workspace\rust\hello_world>cargo run
   Compiling hello_world v0.1.0 (C:\Users\jiangbo1446\work\workspace\rust\hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 1.82s
     Running `target\debug\hello_world.exe`
Hello, world!

```

## 总结

使用 Cargo 新建了一个 Hello World 程序。

## 附录

[1]:xxx
