# 【Rust】标准库-恐慌

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std/panic.html>  

## 示例

在发生恐慌的时候，也可以保住分配的内存被释放。

### main.rs

```rust
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

fn main() {
    // 分配内存
    let _x = Box::new(0i32);
    division(3, 0);
    println!("This point won't be reached!");
    // 释放内存
}
```

## 总结

了解了 Rust 中恐慌的使用，在发生 panic 后，内存可以被正确回收。

## 附录
