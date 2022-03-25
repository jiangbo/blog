# 【Rust】可选枚举（一）

## 环境

- Time 2022-03-25
- Rust 1.59.0

## 示例

### is_some

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.is_some());
}
```

### is_none

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.is_none());
}
```

### as_ref

```rust
fn main() {
    let name = Some("JiangBo".to_owned());
    println!("{:?}", name.as_ref().map(|e| e.len()));
    println!("{:?}", name);
}
```

### as_mut

```rust
fn main() {
    let mut name = Some("JiangBo".to_owned());
    name.as_mut().map(|e| e.push('!')).unwrap();
    println!("{:?}", name);
}
```

### expect

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.expect("Error"));
}
```

### unwrap

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.unwrap());
}
```

### unwrap_or

```rust
fn main() {
    let name: Option<&str> = None;
    println!("{:?}", name.unwrap_or("Rust"));
}
```

### unwrap_or_else

```rust
fn main() {
    let age: Option<i32> = None;
    println!("{:?}", age.unwrap_or_else(|| 2 * 4));
}
```

### unwrap_or_default

```rust
fn main() {
    let age: Option<i32> = None;
    println!("{:?}", age.unwrap_or_default());
}
```

## 总结

了解了可选枚举中相关的一些方法。

## 附录
