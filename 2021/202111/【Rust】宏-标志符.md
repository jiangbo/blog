# 【Rust】宏-标志符

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/macros/designators.html>  

## 示例

以下是可以使用的标志符：

- `block`
- `expr` 表达式
- `ident` 变量名或函数名
- `item`
- `literal` 字面量
- `pat` 模式
- `path`
- `stmt` 语句
- `tt` 标记树
- `ty` 类型
- `vis` 可见性修饰符

### main.rs

```rust
macro_rules! create_function {
    // 使用了 iden 标志符，参数名以$开头
    ($func_name:ident) => {
        // 将传入的参数名作为了函数名
        fn $func_name() {
            // `stringify!` 将 iden 传为字符串
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// 使用宏创建了两个函数
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // expr 表示表达式
    ($expression:expr) => {
        // $expression求结果
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}
```

## 总结

了解了 Rust 中带有参数的宏的定义，参数可以有不同的类型。

## 附录
