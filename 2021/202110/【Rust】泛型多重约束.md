# 【Rust】泛型多重约束

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/generics/multi_bounds.html>  

## 示例

泛型除了使用单一约束外，也可以使用多重约束，也叫多重边界，使用加号（+）连接多个约束。

### main.rs

```rust
use std::fmt::{Debug, Display};

// 要同时实现 Debug 和 Display
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];

    compare_prints(&string);
    // 编译错误，因为 array 只实现了 Debug，没有实现 Display
    // compare_prints(&array);
}
```

## 总结

了解了 Rust 中的多重约束，使用加号连接多个约束，表示需要同时满足所有的约束。

## 附录
