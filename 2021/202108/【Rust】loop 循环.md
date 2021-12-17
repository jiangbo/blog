# 【Rust】loop 循环

## 环境

- Rust 1.56.1
- VSCode 1.60.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/flow_control/loop.html>  

使用 `loop` 关键字来定义一个无限循环。

## 示例

### loop 循环

在 `loop` 循环中，可以使用 `break` 跳出循环，使用 `continue` 继续下次循环。

```rust
fn main() {
    let mut count = 0u32;
    println!("Let's count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // 跳过循环
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            // 退出循环
            break;
        }
    }
}
```

### 嵌套循环和标签

如果有多层嵌套循环需要跳过或者退出，可以使用标签。

```rust
fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            // 可以直接跳出外层循环
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}
```

### 返回值

若要在循环中返回一个值，可以放在 `break` 后面。

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // 直接在这里返回值
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}
```

## 总结

了解了 Rust 中的 loop 循环，可以使用多层循环，也可以在循环中返回值。

## 附录
