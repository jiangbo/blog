# 【Rust】迭代器-any

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/fn/closures/closure_examples/iter_any.html>  

## 示例

前面学习了闭包，现在看看标准库中的例子：Iterator::any。

### Iterator

```rust
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `Self::Item` states it takes
        // arguments to the closure by value.
        F: FnMut(Self::Item) -> bool {}
}
```

### Vector

```rust
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 迭代为 i32 的引用，使用了解构。
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // 直接捕获值
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
}
```

### Array

```rust
fn main() {
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 解构
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // 移动
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}
```

## 总结

了解了 Rust 中迭代器的 any 方法使用闭包的方式。

## 附录
