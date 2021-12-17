# 【Rust】String 转换

## 环境

- Rust 1.56.1
- VSCode 1.60.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/conversion/string.html>  

## 示例

### 转为字符串

要将任何类型转为 String 类型，只需要实现 `toString` trait 就可以了。

```rust
struct Circle {
    radius: i32,
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
```

### Display

除了可以直接实现 `ToString` 外，还可以实现 `Display` 来自动获得 `ToString`，这样还可以使用 `print!` 直接输出。

```rust
use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    println!("{}", circle);
}
```

### 从字符串转换

要从字符串转为其它类型，可以实现 `FromStr`，和 `From` 类似。

```rust
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Circle { radius: s.parse()? })
    }
}

fn main() {
    println!("{:?}", Circle::from_str("44").unwrap());
}
```

### parse

实现了 `FromStr` ，就可以通过 `parse` 方法来直接进行转换。

```rust
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Circle { radius: s.parse()? })
    }
}

fn main() {
    let num: u32 = "44".parse().unwrap();
    println!("{}", num);
    let num = "44.44".parse::<f64>().unwrap();
    println!("{}", num);
    println!("{:?}", Circle::from_str("44").unwrap());
    let circle: Circle = "44".parse().unwrap();
    println!("{:?}", circle);
}
```

## 总结

了解了 Rust 中的字符串的转换，实现了 `ToString`，`Display`，`FromStr` 等。

## 附录
