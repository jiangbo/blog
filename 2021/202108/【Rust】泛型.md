# 【Rust】泛型

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/generics.html>  

## 示例

Rust 中的泛型和其它语言类似，在定义的时候不能确定具体的类型，等到具体实现时才能确定类型。泛型使用尖括号（<>）定义。

### main.rs

```rust
struct A;
// 定义了泛型 T
struct SingleGen<T>(T);

fn main() {
    // 泛型的实现 A
    let _t = SingleGen(A);
    // 泛型的实现，i32
    let _i32 = SingleGen(6);
    // 泛型的实现，char
    let _char = SingleGen('a');
}
```

## 总结

了解了 Rust 中怎么定义泛型和使用泛型。

## 附录
