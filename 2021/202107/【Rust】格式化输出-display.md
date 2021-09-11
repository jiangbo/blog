# 【Rust】格式化输出-display

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

Display 输出使用 `{}` 来进行打印，Display 主要是面向用户的输出。
如果要实现 display 输出，需要实现 `std::fmt::Display` 这个 trait。

> trait 可以先简单理解为其它编程语言中的接口

## 示例

### display 输出

```rust
fn main() {
    let a = "name";
    let b = 44;

    println!("{}, {}", a, b);
}
```

### 手动实现

```rust
fn main() {
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let origin = Point { x: 0, y: 0 };
    println!("{}", origin);
}
```

### 练习

```rust
fn main() {
    use std::fmt;

    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Display: {} + {}i", self.x, self.y)
        }
    }

    impl fmt::Debug for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
        }
    }

    let origin = Point2D { x: 3.3, y: 7.2 };
    println!("{}", origin);
    println!("{:?}", origin);
}
```

## 总结

介绍了使用 Display 进行格式化输出。

## 附录
