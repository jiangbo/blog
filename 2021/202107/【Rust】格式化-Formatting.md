# 【Rust】格式化-Formatting

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

格式化练习，参考资料：<https://doc.rust-lang.org/stable/rust-by-example/hello/print/fmt.html>

## 示例

```rust
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // RGB (128, 255, 90) 0x80FF5A
        write!(
            f,
            "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}",
            self.red, self.green, self.blue
        )
    }
}

fn main() {
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", *color)
    }
}
```

## 总结

格式化输出的练习。

## 附录
