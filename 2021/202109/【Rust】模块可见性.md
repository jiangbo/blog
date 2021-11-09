# 【Rust】模块可见性

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/mod/visibility.html>  

## 示例

默认情况下，模块中的项都是私有有的，可以通过 `pub` 关键字来进行公开。可以使用 `mod` 关键字定义模块。

### 公开函数

```rust
// 定义一个 my_mode 的模块
mod my_mod {
    //定义一个公开的函数
    pub fn function() {
        println!("called `my_mod::function()`");
    }
}

fn main() {
    // 双冒号访问
    my_mod::function();
}
```

### 私有函数

```rust
// 定义一个 my_mode 的模块
mod my_mod {
    //定义一个公开的函数
    pub fn function() {
        println!("called `my_mod::function()`");
        // 同一个模块不受公共和私有的限制
        private_function();
    }

    // 默认私有
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }
}

fn main() {
    // 双冒号访问
    my_mod::function();
    // 编译错误，私有的不能访问
    // my_mod::private_function();
}
```

### 当前 crate 可用

```rust
// 定义一个 my_mode 的模块
mod my_mod {
    // `pub(crate)` 使得函数只在当前 crate 中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }
}

fn main() {
    // 双冒号访问
    my_mod::public_function_in_crate();
}
```

## 总结

了解了 Rust 中模块的可见性问题，默认是私有的，如果需要公开使用 `pub` 关键字。

## 附录
