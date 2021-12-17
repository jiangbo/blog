# 【Rust】使用库文件

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/crates/using_lib.html>  

## 示例

要链接到指定的库，可以通过 `--extern` 参数来指定。

### rustc 链接库

```rust
fn main() {
    rary::public_function();
    // 编译错误，私有方法
    //rary::private_function();
    rary::indirect_access();
}
```

```text
PS C:\Users\\work\workspace\rust\rust\src> rustc main.rs --extern rary=library.rlib
PS C:\Users\\work\workspace\rust\rust\src> .\main.exe
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```

### cargo

建立如下的文件结构：

```text
│  Cargo.toml
├─rary
│  │  Cargo.toml
│  └─src
│          lib.rs
└─src
        main.rs
```

lib.rs 中的内容：

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

main.rs 中的内容：

```rust
fn main() {
    rary::public_function();
    rary::indirect_access();
}
```

cargo.toml

```toml
[dependencies]
rary={path="rary"}
```

## 总结

了解了 Rust 中使用库文件的方式，通过 rustc 可以直接链接到库文件，而 cargo 可以将两个项目链接到一起。

## 附录
