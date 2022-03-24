# 【Rust】字符串切片（三）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### lines

```rust
fn main() {
    let name = "Jiang\nBo";
    name.lines().for_each(|e| println!("{e:?}"));
}
```

### encode_utf16

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.encode_utf16().count());
}
```

### contains

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.contains("ngB"));
}
```

### starts_with

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.starts_with("Ji"));
}
```

### ends_with

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.ends_with("Bo"));
}
```

### find

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.find("Bo"));
}
```

### rfind

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.rfind("Bo"));
}
```

### split

```rust
fn main() {
    let name = "Jiang\nBo";
    name.split('\n').for_each(|e| println!("{e:?}"));
}
```

### split_inclusive

```rust
fn main() {
    let name = "Jiang\nBo";
    name.split_inclusive('\n').for_each(|e| println!("{e:?}"));
}
```

## 总结

了解了字符串切片中相关的一些方法。

## 附录
