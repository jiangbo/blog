# 【Rust】结构体-square

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html>

Add a function `square` which takes a `Point` and a `f32` as arguments, and returns a `Rectangle` with its lower left corner on the point, and a width and height corresponding to the `f32`.

增加一个函数 square，接受的参数是一个 Point 和一个 f32，并返回一个 Rectangle（长方形），其左下角的点等于 Point 参数，长和宽都等于 f32 参数。

## 示例

```rust
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn square(point: &Point, length: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y + length,
        },
        bottom_right: Point {
            x: point.x + length,
            y: point.y,
        },
    }
}

fn main() {
    println!("square: {:?}", square(&Point { x: 0.0, y: 0.0 }, 1.0));
}
```

## 总结

根据要求，实现生成长方形的方法。

## 附录
