# 【Rust】宏-重载

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/macros/overload.html>  

## 示例

### main.rs

```rust
macro_rules! test {
    ($left:expr ;and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // 每个分支需要使用分号结尾
    ($left:expr ;or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
```

## 总结

了解了 Rust 中使用宏时的重载。

## 附录
