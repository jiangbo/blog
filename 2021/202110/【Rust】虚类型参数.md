# 【Rust】虚类型参数

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/generics/phantom.html>  

## 示例

虚类型参数不会在运行时出现，仅在编译时进行静态类型检查的类型参数。

### main.rs

```rust
use std::marker::PhantomData;

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// 会为类型 A 分配存储空间，但是 B 不会

fn main() {
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 编译错误，类型不同
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);

    // 编译错误，类型不同
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}
```

## 总结

了解了 Rust 中的虚类型参数，这个只在编译时进行类型检查，并不带到运行时，也不分配存储空间。

## 附录
