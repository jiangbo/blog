# 【Rust】函数

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/fn.html>  

## 示例

### 定义函数

使用关键字 `fn` 定义函数，函数定义和顺序无关。

```rust
fn main() {
    say()
}

// main 函数虽然在 say 的前面定义，但还是可以使用。
fn say() {
    println!("hello world")
}
```

### 函数返回值

函数可以定义返回值，如果没有默认就是单元类型。

```rust
fn main() {
    println!("add one: {}", add_one(6));
}

fn add_one(i: i32) -> i32 {
    i + 1
}
```

### FizzBuzz

```rust
fn main() {
    for n in 1..=100 {
        fizzbuzz(n);
    }
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}
```

## 总结

了解了 Rust 中的函数，可以使用 `fn` 定义函数，可以有参数和返回值。

## 附录
