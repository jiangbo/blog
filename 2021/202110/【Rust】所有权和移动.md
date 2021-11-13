# 【Rust】所有权和移动

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/scope/move.html>  

## 示例

因为变量要负责释放他们的资源，所有一个资源只能有一个所有者，避免重复释放，其中引用不拥有资源。在进行赋值的时候，所有权会发生转移，这称为移动（move），发生移动后，之前的变量不能再使用了，这可以避免悬垂指针（dangling pointers）。

### main.rs

```rust
// 获取所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
    // c 在这里被销毁
}

fn main() {
    // 栈上分配
    let x = 5u32;

    // 没有发生移动，是拷贝（copy）
    let y = x;

    // 两个变量同时可用
    println!("x is {}, and y is {}", x, y);

    // 堆上分配
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // 发生了移动
    let b = a;

    // 编译错误，a 已经发生了移动
    // println!("a contains: {}", a);

    // 将所有权传递到了函数中，也发生了移动
    destroy_box(b);

    // 编译错误，发生了移动，已经不能再使用了
    // println!("b contains: {}", b);
}
```

## 总结

了解了 Rust 中的变量的所有权和移动，如果没有实现 Copy，赋值和传参的时候，会转移所有权，称为移动。

## 附录
