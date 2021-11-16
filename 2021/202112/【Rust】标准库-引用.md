# 【Rust】标准库-引用

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std/rc.html>  

## 示例

rust 使用 `Rc` 来实现引用计数。

### main.rs

```rust
use std::rc::Rc;

fn main() {
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");

        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        {
            println!("--- rc_a is cloned to rc_b ---");

            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        println!("--- rc_a is dropped out of scope ---");
    }
    // 编译错误
    // println!("rc_examples: {}", rc_examples);
}
```

## 总结

了解了 Rust 中标准库中的 Rc 的使用。

## 附录
