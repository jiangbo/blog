# 【Rust】trait-派生

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/trait/derive.html>  

## 示例

编译器可以通过 `#[derive]` 属性提供派生（自动实现）功能，以下 trait 支持派生：

- 比较 trait: `Eq`，`PartialEq`，`Ord`，`PartialOrd`
- `Clone`，用来从 `&T` 创建副本 `T`
- `Copy`，使类型具有 “复制语义”而非 “移动语义”。
- `Hash`，从 `&T` 计算哈希值（hash）。
- `Default`，创建数据类型的一个空实例。
- `Debug`，使用 `{:?}` formatter 来格式化一个值。

### main.rs

```rust
// 自动实现比较的 trait
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// 自动实现 Debug trait
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // 编译错误，没有实现 Debug trait
    // println!("One second looks like: {:?}", _one_second);

    // 编译错误，没有实现比较的 trait
    // let _this_is_true = (_one_second == _one_second);

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {} than one meter.", cmp);
}
```

## 总结

了解了 Rust 中 trait 的派生，可以使用属性来自动生成一个 trait 的实现。

## 附录
