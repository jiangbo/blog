# 【Rust】while-let

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/while_let.html>  

## 示例

### loop-match

```rust
#![allow(unused)]
fn main() {
    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            }
            // 看起来这个没有什么用
            _ => {
                break;
            }
        }
    }
}
```

### while-let

```rust
fn main() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // 这里没有多余的代码，缩进也更短了
    }
}
```

## 总结

了解了 Rust 中 `while-let` 语法，有时候可以优化 `loop-match` 循环。

## 附录
