# 【Rust】字符串（四）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### drain

```rust
fn main() {
    let mut name = String::from("JiangBo");
    let name = name.drain(..5);
    println!("{:?}", name);
}
```

### replace_range

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.replace_range(5.., "Ob");
    println!("{:?}", name);
}
```

### into_boxed_str

```rust
fn main() {
    let name = String::from("JiangBo");
    let name = name.into_boxed_str();
    println!("{:?}", name);
}
```

### from_utf16

```rust
fn main() {
    let value = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0x0069, 0x0063];
    let value = String::from_utf16(value).unwrap();
    println!("{:?}", value);
}
```

### from_utf16_lossy

```rust
fn main() {
    let value = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0x0069, 0x0063];
    let value = String::from_utf16_lossy(value);
    println!("{:?}", value);
}
```

### try_reserve

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.try_reserve(10).unwrap();
    println!("{:?}", name.capacity());
}
```

### try_reserve_exact

```rust
fn main() {
    let mut name = String::from("JiangBo");
    name.try_reserve_exact(10).unwrap();
    println!("{:?}", name.capacity());
}
```

## 总结

了解了字符串中相关的一些方法。

## 附录
