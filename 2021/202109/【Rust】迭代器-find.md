# 【Rust】迭代器-find

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/fn/closures/closure_examples/iter_find.html>  

## 示例

前面学习了闭包，现在看看标准库中的例子：Iterator::find。

### Iterator

```rust
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool {}
}
```

### Vector

```rust
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut into_iter = vec2.into_iter();

    // 解构，第一层是 iter 的引用，第二层是 find 的引用
    let v1 = vec1.iter().find(|&&x| x == 2);
    println!("Find 2 in vec1: {:?}", v1);
    // 解构，是 find 的引用
    let v2 = into_iter.find(|&x| x == 2);
    println!("Find 2 in vec2: {:?}", v2);
}
```

### Array

```rust
fn main() {
    let arr1 = [1, 2, 3];
    let arr2 = [4, 5, 6];

    let mut into_iter = arr2.into_iter();

    // 解构，第一层是 iter 的引用，第二层是 find 的引用
    let a1 = arr1.iter().find(|&&x| x == 2);
    println!("Find 2 in vec1: {:?}", a1);
    // 解构，是 find 的引用
    let a2 = into_iter.find(|&x| x == 2);
    println!("Find 2 in vec2: {:?}", a2);
}
```

### 位置索引

```rust
fn main() {
    let vec = vec![1, 9, 3, 3, 13, 2];

    let v1 = vec.iter().position(|x| x % 2 == 0);
    assert_eq!(v1, Some(5));

    let v2 = vec.iter().position(|x| x < &0);
    assert_eq!(v2, None);
}
```

### find_map

```rust
fn main() {
    let a = ["lol", "NaN", "2", "5"];
    let first_number = a.iter().find_map(|s| s.parse().ok());
    assert_eq!(first_number, Some(2));
}
```

## 总结

了解了 Rust 中迭代器的 find 方法使用闭包的方式。

## 附录
