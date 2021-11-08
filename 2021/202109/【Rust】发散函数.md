# 【Rust】发散函数

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/fn/diverging.html>  

## 示例

发散函数没有返回类型，使用 `!` 标记，和单元类型不一样。

### panic

```rust
#![allow(unused)]
fn main() {
    fn foo() -> ! {
        panic!("This call never returns.");
    }
}
```

### continue

```rust
fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 返回类型是 u32
            let addition: u32 = match i % 2 == 1 {
                true => i,
                // continue 没有返回任何值
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );
}
```

## 总结

了解了 Rust 中的发散函数，发散函数是指没有返回值的函数。

## 附录
