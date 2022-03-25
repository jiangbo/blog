# 【Rust】可选枚举（二）

## 环境

- Time 2022-03-25
- Rust 1.59.0

## 示例

### unwrap_unchecked

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", unsafe { name.unwrap_unchecked() });
}
```

### map

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.map(|e| e.len()));
}
```

### map_or

```rust
fn main() {
    let name = None;
    println!("{:?}", name.map_or(4, |e: &str| e.len()));
}
```

### map_or_else

```rust
fn main() {
    let name = None;
    println!("{:?}", name.map_or_else(|| 4, |e: &str| e.len()));
}
```

### ok_or

```rust
fn main() {
    let name: Option<&str> = None;
    println!("{:?}", name.ok_or(44));
}
```

### ok_or_else

```rust
fn main() {
    let name: Option<&str> = None;
    println!("{:?}", name.ok_or_else(|| 44));
}
```

### as_deref

```rust
fn main() {
    let name = Some("JiangBo".to_owned());
    println!("{:?}", name.as_deref());
}
```

### as_deref_mut

```rust
fn main() {
    let mut name = Some("JiangBo".to_owned());
    name.as_deref_mut().map(|e| {
        e.make_ascii_lowercase();
        e
    });
    println!("{:?}", name);
}
```

### iter

```rust
fn main() {
    let name = Some("JiangBo".to_owned());
    println!("{:?}", name.iter().next());
}
```

## 总结

了解了可选枚举中相关的一些方法。

## 附录
