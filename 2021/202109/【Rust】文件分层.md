# 【Rust】文件分层

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/mod/split.html>  

## 示例

将所有的代码写到同一个文件太对的时候，可以考虑分别写到不同的文件中，文件层级如下：

```text
$ tree .
.
|-- my
|   |-- inaccessible.rs
|   |-- mod.rs
|   `-- nested.rs
`-- main.rs
```

### main.rs

```rust
// 这个会去找 mod.rs 或者 my/mod.rs 文件，并插入到这个地方。
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();
    function();
    my::indirect_access();
    my::nested::function();
}
```

### mod.rs

```rust
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}
```

### inaccessible.rs

```rust
#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}
```

### nested.rs

```rust
pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}
```

## 总结

了解了 Rust 中模块的文件组成结构，当代码量增大的时候，可以考虑放到不同的文件中。

## 附录
