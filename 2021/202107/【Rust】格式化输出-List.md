# 【Rust】格式化输出-List

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

格式化练习，参考资料：<https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display/testcase_list.html>

## 示例

```rust
use std::fmt; // Import the `fmt` module.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```

## 总结

格式化输出 List 的练习。

## 附录
