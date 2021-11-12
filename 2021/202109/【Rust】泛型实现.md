# 【Rust】泛型实现

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/generics/impl.html>  

## 示例

### 泛型的定义

```rust
#![allow(unused)]
fn main() {
    //  一个普通的结构体
    struct S;
    // 定义一个带有泛型的结构体
    struct GenericVal<T>(T);

    // 实现泛型，类型为 f32
    impl GenericVal<f32> {}
    // 实现泛型，类型为 S 结构体
    impl GenericVal<S> {}

    // 定义泛型去实现了另一个泛型
    impl<T> GenericVal<T> {}
}
```

### 泛型的使用

```rust
struct GenVal<T> {
    gen_val: T,
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let y = GenVal { gen_val: 3i32 };
    println!("y = {}", y.value());
}
```

## 总结

了解了 Rust 中怎么定义泛型和怎么使用泛型。

## 附录
