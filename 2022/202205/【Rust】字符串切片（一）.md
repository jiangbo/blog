# 【Rust】字符串切片（一）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### len

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.len());
}
```

### is_empty

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.is_empty());
}
```

### is_char_boundary

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.is_char_boundary(5));
}
```

### as_bytes

```rust
fn main() {
    let name = "JiangBo";
    let name = name.as_bytes();
    println!("{:?}", name);
}
```

### as_bytes_mut

```rust
fn main() {
    let mut name = String::from("JiangBo");
    let name = unsafe { name.as_bytes_mut() };
    println!("{:?}", name);
}
```

### as_ptr

```rust
fn main() {
    let name = "JiangBo";
    println!("{:p}", name.as_ptr());
}
```

### as_mut_ptr

```rust
fn main() {
    let mut name = String::from("JiangBo");
    println!("{:p}", name.as_mut_ptr());
}
```

### get

```rust
fn main() {
    let name = "JiangBo";
    println!("{:?}", name.get(5..));
}
```

### get_mut

```rust
fn main() {
    let mut name = String::from("JiangBo");
    println!("{:?}", name.get_mut(5..));
}
```

## 总结

了解了字符串中相关的一些方法。

## 附录
