# 【Rust】引用模式

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/ref.html>  

## 示例

取引用可以使用 `&` 和 `ref` 关键字，它们有各自的使用场景。

### main.rs

```rust
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';

    // 两种使用方式等价
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // 解构时也可以使用引用
    let Point {
        x: ref _ref_to_x,
        y: _,
    } = point;

    let mut mutable_point = point;

    // 可变引用
    let mut_ref_to_y = &mut mutable_point.y;
    *mut_ref_to_y = 1;

    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    let mut mutable_tuple = (Box::new(5u32), 3u32);
    // 可变引用的解构
    let (_, ref mut last) = mutable_tuple;
    *last = 2u32;

    println!("tuple is {:?}", mutable_tuple);
}
```

## 总结

了解了 Rust 中的引用模式，可以通过两种方式取得引用。

## 附录
