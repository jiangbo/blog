# 【Rust】外部函数接口

## 环境

- Rust 1.56.1
- VSCode 1.61.2
- Windows 7
- mingw64 7.3.0

## 概念

参考：<https://stackoverflow.com/questions/40833078/how-do-i-specify-the-linker-path-in-rust>  

## 示例

### Cargo.toml

```toml
[package]
name = "rust"
version = "0.1.0"
edition = "2021"
build = "build.rs"

[dependencies]
```

### build.rs

该文件放到项目根目录，src 文件夹的上一层。

```rust
fn main() {
    // cargo:rustc-link-search= 后跟上静态库或者动态库的路径，到文件夹
    println!(r"cargo:rustc-link-search=C:\Users\jiangbo\work\rust");
}
```

### main.rs

```rust
#[link(name = "add")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn add_with_c(a: i32, b: i32) -> i32 {
    unsafe { add(a, b) }
}

fn main() {
    let sum = add_with_c(7, 8);
    println!("{}", sum);
}
```

### add.c

```c
int add(int a, int b)
{
    return a + b;
}
```

### 生成库

```shell
gcc -c add.c
ar cr libadd.a add.o
```

## 总结

了解了 Rust 中外部函数接口的使用。

## 附录
