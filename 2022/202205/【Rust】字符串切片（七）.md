# 【Rust】字符串切片（七）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### into_boxed_bytes

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.to_owned().into_boxed_str().into_boxed_bytes());
}
```

### replace

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.replace("gB", "gb"));
}
```

### replacen

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.replacen("gB", "gb", 2));
}
```

### to_lowercase

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.to_lowercase());
}
```

### to_uppercase

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.to_uppercase());
}
```

### into_string

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.to_owned().into_boxed_str().into_string());
}
```

### repeat

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.repeat(4));
}
```

### to_ascii_uppercase

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.to_ascii_uppercase());
}
```

### escape_unicode

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.to_ascii_lowercase());
}
```

## 总结

了解了字符串切片中相关的一些方法。

## 附录
