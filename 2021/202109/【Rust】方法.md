# 【Rust】方法

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/fn/methods.html>  

rust 中有关联函数和方法的概念，关联函数和其它语言中的静态方法类似。而方法和其它语言中的成员方法类似，关联函数使用 `::` 调用，而方法使用 `.` 调用。

## 示例

### 关联函数

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

fn main() {
    let p1 = Point::origin();
    let p2 = Point::new(3.0, 4.0);

    println!("p1: {:?}, p2: {:?}", p1, p2);
}
```

### 方法

```rust
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, p: Point) -> f64 {
        ((self.x - p.x).powi(2) + (self.y - p.y).powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 1.0, y: 1.0 };
    println!("distance: {}", p1.distance(p2))
}
```

### 可变方法

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn move_x(&mut self, x: f64) {
        self.x += x;
    }
}

fn main() {
    let mut p1 = Point { x: 0.0, y: 0.0 };
    p1.move_x(1.0);
    println!("move x: {:?}", p1);
}
```

## 总结

了解了 Rust 中关联函数和方法，关联函数通过类型调用，方法通过类型的实例调用。

## 附录
