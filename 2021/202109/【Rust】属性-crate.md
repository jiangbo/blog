# 【Rust】属性-crate

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/attribute/crate.html>  

## 示例

如果没有使用 cargo，可以使用 `crate_type` 来指定库的类型，使用 `crate_name` 指定名称。
> #![xxx] 这种形式的属性针对整个crate，#[xxx] 这种只针对模块或者项。

### other.rs

```rust
// 定义库类型
#![crate_type = "lib"]
// 定义库名称
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}
```

### 生成库

```text
PS C:\Users\jiangbo\work\workspace\rust\rust\src> rustc .\other.rs
PS C:\Users\jiangbo\work\workspace\rust\rust\src> dir
Mode                LastWriteTime     Length Name
----                -------------     ------ ----
-a---        2021/11/10     20:47      10264 library.rlib
-a---        2021/11/10     20:46        166 other.rs
```

## 总结

了解了 Rust 中怎么使用属性来定义库的名称和类型（在没有使用 cargo 的情况下）。

## 附录
