# 【Rust】use 申明

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/mod/use.html>  

## 示例

use 关键字类似其它语言中的导入，可以将其它模块的函数等导入到当前环境里。

### 导入

```rust
fn main() {
    use std::fmt::{Debug, Display};
    println!("hello world");
}
```

### 重命名

重命名可以使用 `as` 关键字。

```rust
// 重命名为 other_function
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    other_function();
    println!("Entering block");
    {
        // 变量遮蔽
        use crate::deeply::nested::function;
        function();
        println!("Leaving block");
    }
    function();
}
```

## 总结

了解了 Rust 中 use 关键字的使用，可以将其它模块的内容导入到当前环境。

## 附录
