# 【Rust】元组-transpose

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/primitives/tuples.html>

以 `reverse` 函数作为样板，写一个 `transpose` 函数，它可以接受一个 Matrix 作为参数，并返回一个右上-左下对角线上的两元素交换后的 Matrix。举个例子：

```rust
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
```

输出结果：

```text
Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )
```

## 示例

```rust
use std::fmt;

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
```

## 总结

根据要求，实现 `transpose` 函数，反转显示元组中的数据。

## 附录
