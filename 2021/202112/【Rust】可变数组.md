# 【Rust】可变数组

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/std/vec.html>  

## 示例

可变数组（Vector）存储在堆上和普通数组的区别是长度可变。

### main.rs

```rust
fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);
    xs.push(4);
    println!("Vector: {:?}", xs);

    println!("Vector length: {}", xs.len());
    println!("Second element: {}", xs[1]);
    println!("Pop last element: {:?}", xs.pop());

    // 越界会出现 panic
    // println!("Fourth element: {}", xs[3]);

    for x in xs.iter() {
        println!("> {}", x);
    }

    // 可以取得迭代的索引
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
}
```

## 总结

了解了 Rust 中的可变数组，它的长度是可变的。

## 附录
