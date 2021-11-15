# 【Rust】堆栈分配

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std/box.html>  

## 示例

默认情况下，rust 一般都是在栈上进行内存分配，如果想在堆上分配内存，需要使用 `Box<T>`。

### main.rs

```rust
use std::mem;

#[warn(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // 堆上分配
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // 栈上分配
    let point = origin();
    // 堆上分配
    let boxed_point = Box::new(origin());
    // 两层装箱
    let box_in_a_box = Box::new(boxed_origin());

    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );

    // 等于指针的长度
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    // 堆上数据的大小
    let unboxed_point: Point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );
}
```

## 总结

了解了 Rust 中进行堆上分配的方法，可以使用 `Box<T>`，数据的大小在堆栈上没有区别。

## 附录
