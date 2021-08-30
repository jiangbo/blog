# 【Rust】第一个 Rust 程序

## 环境

- Windows 10
- Rust 1.54.0

## Hello World

根据传统，首先编写一个 Hello World 程序。  
参考这里的代码：<https://doc.rust-lang.org/stable/rust-by-example/hello.html>

### 新增 main.rs 文件

新增一个 main.rs 文件，并且在文件中写入如下的内容：

```rs
fn main() {
    println!("Hello World");
}
```

### 编译源代码

Rust 是一种编译型语言，需要经过编译后运行，使用 `rustc` 进行编译。
编译完成后，在相同的目录下，可以找到一个 main.exe 文件

```txt
C:\Users\jiangbo\work\workspace\rust>rustc main.rs
```

### 运行程序

```txt
C:\Users\jiangbo\work\workspace\rust>main
Hello World
```

## 总结

编写了第一个 Rust 程序，并且运行了它，打印出了“Hello World”。

## 附录
