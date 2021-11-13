# 【Rust】多次借用

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/alias.html>  

## 示例

进行借用时，可以以不可变借用多次。但是在不可变借用的时候，不能进行可变借用。同时不支持多次可变借用。

### main.rs

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // 编译错误，后面还在使用不可变借用，不能进行可变借用
    // let mutable_borrow = &mut point;

    println!(
        "Point has coordinates: ({}, {})",
        borrowed_point.x, another_borrow.y
    );

    // 不可变借用使用后，可以进行可变借用
    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;

    // let y = &point.y;

    // 编译错误，还有可变借用，不能进行不可变借用
    // println!("Point Z coordinate is {}", point.y);

    // 可变借用可以当不可变使用
    println!(
        "Point has coordinates: ({}, {})",
        mutable_borrow.x, mutable_borrow.y
    );
    // 可变借用使用完了，可以进行重新借用
    let new_borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {})",
        new_borrowed_point.x, new_borrowed_point.y
    );
}
```

## 总结

了解了 Rust 中的多次借用，有可变借用存在的时候，不能进行不可变借用，不能进行多次可变借用。

## 附录
