# 【Rust】泛型 trait

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/generics/gen_trait.html>  

## 示例

trait 现在还没有一个统一的翻译，之后就直接称呼为 trait，不进行翻译。

### main.rs

```rust
struct Empty;
struct Null;

// 泛型 trait
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// 实现泛型 trait，实现过程还带有泛型
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // 已经进行了移动
    empty.double_drop(null);

    // 编译错误，值已经进行了移动
    // empty;
    // null;
}
```

## 总结

了解了 Rust 中定义和使用泛型 trait。

## 附录
