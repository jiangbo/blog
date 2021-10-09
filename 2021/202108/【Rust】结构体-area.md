# 【Rust】结构体-area

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html>

Add a function `rect_area` which calculates the area of a `Rectangle` (try using nested destructuring).

## 示例

```rust
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } = &self;

        ((x1 - x2) * (y1 - y2)).abs()
    }
}

fn main() {
    let rec = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 2.0, y: 3.0 },
    };

    println!("rec area: {}", rec.rect_area());
}
```

## 总结

根据要求，实现求面积的方法，并使用嵌套的解构。

## 附录
