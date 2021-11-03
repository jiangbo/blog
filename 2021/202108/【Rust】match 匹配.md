# 【Rust】match 匹配

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/match.html>  

Rust 使用 `match` 关键字来进行模式匹配，有点像 C 语言中的 switch。所有可能的分支都需要被覆盖，只会匹配到第一个满足条件的。

## 示例

### 匹配单个值

```rust
fn main() {
    let number = 1;
    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 要覆盖所有的可能分支，不然有编译错误
        _ => println!("Ain't special"),
    }
}
```

### 匹配多个值

```rust
fn main() {
    let number = 7;
    match number {
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 要覆盖所有的可能分支，不然有编译错误
        _ => println!("Ain't special"),
    }
}
```

### 匹配区间

```rust
fn main() {
    let number = 13;
    match number {
        // 匹配区间
        13..=19 => println!("A teen"),
        // 要覆盖所有的可能分支，不然有编译错误
        _ => println!("Ain't special"),
    }
}
```

### 第一个匹配

```rust
fn main() {
    let number = 13;
    match number {
        // 只有第一个匹配的才会执行，后面的不会
        7 | 11 | 13 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}
```

### 返回值

```rust
fn main() {
    let boolean = true;
    // match 是一个表达式，也可以返回值
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}
```

## 总结

了解了 Rust 中的 match 关键字，用来进行模式匹配，和其它语言的 `switch` 类似。

## 附录
