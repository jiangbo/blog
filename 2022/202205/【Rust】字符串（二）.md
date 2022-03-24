# 【Rust】字符串（二）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### capacity

```rust
fn main() {
    let name = String::from("JiangBo");
    println!("{:?}", name.capacity());
}
```

### reserve

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.reserve(10);
    println!("{:?}", name.capacity());
}
```

### reserve_exact

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.reserve_exact(10);
    println!("{:?}", name.capacity());
}
```

### shrink_to_fit

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.reserve_exact(10);
    println!("{:?}", name.capacity());
    name.shrink_to_fit();
    println!("{:?}", name.capacity());
}
```

### shrink_to

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.reserve_exact(10);
    println!("{:?}", name.capacity());
    name.shrink_to(10);
    println!("{:?}", name.capacity());
}
```

### push

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.push('!');
    println!("{name:?}");
}
```

### as_bytes

```rust
fn main() {
    let name = String::from("JiangBo");
    let name = name.as_bytes();
    println!("{name:?}");
}
```

### truncate

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.truncate(5);
    println!("{name:?}");
}
```

### pop

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.pop();
    println!("{name:?}");
}
```

## 总结

了解了字符串中相关的一些方法。

## 附录
