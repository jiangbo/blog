# 【Rust】原始类型-元组

## 环境

- Rust 1.54.0
- VSCode 1.59.1

## 概念

元组（Tuple）是一种组合类型，使用小括号来表示，其中每个值的类型可以不相同。

## 示例

### 类型申明

> Rust 中定义的变量如果不使用的话，可以用下划线开头，就不会有警告信息。

```rust
fn main() {
    let _x = (1, 1.2, true, 'A');
    let _point: (i32, i8) = (0, 0);
}
```

### debug 显示

> 元组默认情况下，只能使用 debug 打印，打印的最大长度是 12。

```rust
fn main() {
    let x = (1, 1.2, true, 'A');
    let point: (i32, i8) = (0, 0);
    println!("x = {:?}", x);
    println!("point = {:?}", point);
}
```

### 取值

根据索引位置可以取值，从 0 开始。

```rust
fn main() {
    let point = (4, 44);
    println!("point.0 = {}, point.1 = {}", point.0, point.1);
}
```

### 解构（destructuring）

```rust
fn main() {
    let point = (4, 44);
    let (x, y) = point;
    println!("x = {}, y = {}", x, y);
}
```

### 单一值元组

如果要定义单一值得元组，逗号不能省略。

```rust
fn main() {
    let a = (4,);
    println!("a = {:?}", a);
}
```

### 嵌套

```rust
fn main() {
    let a = (4, (4, 4));
    println!("a = {:?}", a);
}
```

### 单元值

没有任何值的元组 `()` 是一种特殊的类型，只有一个值，也写成 `()` 。  
该类型被称为 **单元类型**（_unit type_），而该值被称为 **单元值**（_unit value_）。  
如果表达式不返回任何其他值，则会隐式返回单元值。

## 总结

介绍了 Rust 中元组的概念和一些使用方式。

## 附录
