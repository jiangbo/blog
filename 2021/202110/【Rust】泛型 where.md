# 【Rust】泛型 where

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/generics/where.html>  

## 示例

泛型约束除了可以写在泛型定义的后面，还可以使用 where 子句来表达，这样更具表现力。

### where

```rust
trait TraitB {}
trait TraitC {}
trait TraitE {}
trait TraitF {}

trait MyTrait1<A, D> {}

trait MyTrait2<A, D> {}

struct YourType;

impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait1<A, D> for YourType {}

// where 使用起来更短，更具表现力
impl<A, D> MyTrait2<A, D> for YourType
where
    A: TraitB + TraitC,
    D: TraitE + TraitF,
{
}

fn main() {}
```

### 约束其它类型

```rust
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// 直接限制的 Option<T> 的类型而不是 T
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
```

## 总结

了解了 Rust 中 where 来限制泛型的类型，如果类型少，简单可以直接定义，如果多和复杂，可以考虑使用 where。

## 附录
