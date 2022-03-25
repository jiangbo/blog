# 【Rust】可选枚举（三）

## 环境

- Time 2022-03-25
- Rust 1.59.0

## 示例

### iter_mut

```rust
fn main() {
    let mut name = Some("JiangBo".to_owned());
    println!("{:?}", name.iter_mut().next());
}
```

### and

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.and(Some("Rust")));
}
```

### and_then

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.and_then(|e| Some(e.len())));
}
```

### filter

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.filter(|e| e.is_empty()));
}
```

### or

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.or(None));
}
```

### or_else

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.or_else(|| None));
}
```

### xor

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.xor(None));
}
```

### insert

```rust
fn main() {
    let mut name = Some("JiangBo");
    println!("{:?}", name.insert("Rust"));
}
```

### get_or_insert

```rust
fn main() {
    let mut name = Some("JiangBo");
    println!("{:?}", name.get_or_insert("Rust"));
}
```

## 总结

了解了可选枚举中相关的一些方法。

## 附录
