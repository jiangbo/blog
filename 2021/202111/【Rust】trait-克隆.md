# 【Rust】trait-克隆

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/rust-by-example/trait/clone.html>  

## 示例

### main.rs

```rust
#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let unit = Unit;
    // 直接使用 copy
    let copied_unit = unit;

    // 两个都能使用
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // 使用的移动语义
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // 编译错误，值已经移动
    // println!("original: {:?}", pair);

    // 克隆一个新的
    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    // 克隆出来的还是可以正常使用
    println!("clone: {:?}", cloned_pair);
}
```

## 总结

了解了 Rust 中 clone，需要使用派生自动生成 clone 方法。

## 附录
