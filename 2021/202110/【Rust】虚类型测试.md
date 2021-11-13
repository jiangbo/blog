# 【Rust】虚类型测试

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://rust-by-example.budshome.com/generics/phantom/testcase_units.html>  

## 示例

### main.rs

```rust
use std::marker::PhantomData;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<U>(f64, PhantomData<U>);

impl<U> Add for Length<U> {
    type Output = Length<U>;

    fn add(self, rhs: Length<U>) -> Length<U> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // 编译错误，类型不匹配
    // let one_feter = one_foot + one_meter;
}
```

## 总结

了解了 Rust 中的虚类型参数。

## 附录
