# 【Rust】元组-display

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/primitives/tuples.html>

给 Matrix `结构体` 加上 `fmt::Display` trait，这样当你从 Debug 格式化 `{:?}` 切换到 Display 格式化 `{}` 时，会得到如下的输出：

```text
( 1.1 1.2 )
( 2.1 2.2 )
```

## 示例

```rust
use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
}
```

## 总结

根据要求，实现了 `Display` trait，并按要求进行了显示。

## 附录
