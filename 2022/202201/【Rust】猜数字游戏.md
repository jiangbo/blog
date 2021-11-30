# 【Rust】猜数字游戏

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html>  

## 示例

### main.rs

```rust
use std::{cmp::Ordering, io::stdin};

use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("请输入一个数字：");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if let Ok(input) = input.trim().parse::<i32>() {
            match input.cmp(&num) {
                Ordering::Less => println!("太小了"),
                Ordering::Greater => println!("太大了"),
                Ordering::Equal => {
                    println!("猜中了");
                    break;
                }
            }
        }
    }
}
```

## 总结

使用 rust 实现了一个猜数字游戏。

## 附录
