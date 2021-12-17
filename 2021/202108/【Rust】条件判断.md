# 【Rust】条件判断

## 环境

- Rust 1.56.1
- VSCode 1.60.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/if_else.html>  

Rust 中的 if/else 条件判断，条件可以不使用小括号，但是后面的代码需要使用大括号。

## 示例

### if/else

```rust
fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }
}
```

### if 表达式

使用 if 表达式的时候，if/else 需要返回相同的类型。

```rust
fn main() {
    let n = 5;
    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        // 返回 i32 类型
        10 * n
    } else {
        println!(", and is a big number, halve the number");

        // else 中也需要返回 i32 类型
        n / 2
        // 如果在这里加上了分号，返回类型就不一致了，会出现编译错误
    };
    // 最后的分号不能省

    println!("{} -> {}", n, big_n);
}
```

## 总结

了解了 Rust 中的条件判断，可以使用 if/else 来判断条件，当有返回类型时，所有返回类型需要一致。

## 附录
