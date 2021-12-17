# 【Rust】定义新类型

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/generics/new_types.html>  

## 示例

如果直接使用基础类型容易误导和出错，可以考虑定义一种新的类型，来提供编译保证。

### main.rs

```rust
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // 编译错误，类型不匹配
    // println!("Old enough {}", old_enough(&age_days));

    let _years1: i64 = age.0; // 元组取值
    let Years(_years2) = age; // 解构
}
```

## 总结

了解了 Rust 中，在适当的时候，可以考虑定义新的类型来提供编译保证。

## 附录
