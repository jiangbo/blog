# 【Rust】创建库文件

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/crates/lib.html>  

## 示例

create 是 rust 中的一个编译单元，和其它语言的库的概念类似。模块并不会单独编译，只有 crate 才会。默认情况下，create 生成二进制文件，但可以自己指定。

### rustc

使用 `rustc --crate-type=lib rary.rs` 生成库文件，其中 rary.rs 是文件名。

```rust
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

### cargo

除了通过 rustc 直接编译，也可以通过 cargo 来创建。通过 `cargo new rary --lib --vcs=none` 命令创建新项目。

```text
PS C:\Users\jiangbo1446\work\workspace\rust\rust> cargo new rary --lib --vcs=none
     Created library `rary` package
```

在 lib.rs 中填写：

```rust
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

使用命令 `cargo build --release` 生成。

## 总结

了解了 Rust 中生成一个库文件的方式，可以通过 rustc 编译直接生成，也可以通过 cargo 来创建。

## 附录
