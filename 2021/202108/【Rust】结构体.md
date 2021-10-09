# 【Rust】结构体

## 环境

- Rust 1.55.0
- VSCode 1.59.1

## 概念

结构体（structure，缩写成 struct）有 3 种类型，使用 `struct` 关键字来创建：

- 元组结构体（tuple struct），事实上就是具名元组。
- 经典的 C 语言风格结构体。
- 单元结构体（unit struct），不带字段，在泛型中很有用。

## 示例

### 单元结构体

```rust
#[derive(Debug)]
struct Unit;

fn main() {
    let unit = Unit;
    println!("{:?}", unit)
}
```

### 元组结构体

```rust
struct Pair(i32, f32);

fn main() {
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 元组结构体的解构
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}
```

### 经典 C 结构体

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);
    println!("person name: {}, person age: {}", peter.name, peter.age);
}
```

### C 解构体赋值和嵌套

```rust
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 赋值
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 解构赋值
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    println!("{}", left_edge);
    println!("{}", top_edge);

    // 解构嵌套
    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
}
```

## 总结

了解了 Rust 中的结构体，有三种形式的结构体，分别是单元结构体，元组结构体和经典 C 结构体。  
对三种结构体进行了初始化和简单的赋值和使用。

## 附录
