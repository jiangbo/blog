# 【Rust】相对路径

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/mod/super.htmll>  

## 示例

`super` 和 `self` 关键字可以使用在导入模块的时候，就不用指定绝对路径而是使用相对路径。

### self

```rust
mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // 调用自己模块中的函数，和不写的结果一样。
        self::function();
        // 使用自己模块下面的模块，和不写的结果一样。
        self::cool::function();
    }
}

fn main() {
    my::indirect_call();
}
```

### super

```rust
fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {

    pub fn indirect_call() {
        //  相对路径
        super::function();

        {
            // 绝对路径
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
```

## 总结

了解了 Rust 中导入模块可以使用 `super` 和 `self` 关键字来导入模块。

## 附录
