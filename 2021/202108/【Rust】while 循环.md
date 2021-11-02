# 【Rust】while 循环

## 环境

- Rust 1.56.1
- VSCode 1.60.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/while.htmll>  

while 循环在条件为真的时候，一直执行，直到为假。

## 示例

### while 循环

```rust
fn main() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}
```

## 总结

了解了 Rust 中的 while 循环。

## 附录
