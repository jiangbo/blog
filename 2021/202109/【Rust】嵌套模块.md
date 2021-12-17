# 【Rust】嵌套模块

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/mod/visibility.html>  

## 示例

### 模块嵌套

```rust
mod my_mod {
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }
    }
    // 私有模块
    mod nested1 {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }
    }
}

fn main() {
    my_mod::nested::function();
    // 私有模块不能访问
    // my_mod::nested1::function();
}
```

### in crate

```rust
mod my_mod {

    pub fn function() {
        use nested::public_function_in_my_mod;
        public_function_in_my_mod();
    }

    pub mod nested {
        // 只能在 create::my_mod 模块中访问
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`");
        }
    }
}

fn main() {
    my_mod::function();
}
```

### self

```rust
mod my_mod {
    pub mod nested {
        //  `pub(self)` 语法表示只能在当前模块访问和私有一样
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }
    }
}

fn main() {
    // 编译错误，不能访问
    // my_mod::nested::public_function_in_nested();
}
```

### super

```rust
mod my_mod {
    use self::nested::public_function_in_super_mod;

    pub fn function() {
        public_function_in_super_mod();
    }
    pub mod nested {
        // 只能在父模块访问
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }
}

fn main() {
    my_mod::function();
}
```

## 总结

了解了 Rust 中模块的嵌套和各种访问性的问题。

## 附录
