# 【Rust】可变性

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/scope/move/mut.html>  

## 示例

在发生所有权转移时，数据的可变性可能发生改变。

### main.rs

```rust
fn main() {
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);

    // 编译错误，不可变变量
    //*immutable_box = 4;

    // 移动，改变了可变性
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    // 修改值
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}
```

## 总结

了解了 Rust 中发生移动时，变量的可变性可能会发生改变。

## 附录
