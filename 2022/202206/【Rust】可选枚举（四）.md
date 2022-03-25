# 【Rust】可选枚举（四）

## 环境

- Time 2022-03-25
- Rust 1.59.0

## 示例

### get_or_insert_with

```rust
fn main() {
    let mut name = Some("JiangBo");
    println!("{:?}", name.get_or_insert_with(|| "Rust"));
}
```

### take

```rust
fn main() {
    let mut name = Some("JiangBo");
    name.take();
    println!("{:?}", name);
}
```

### replace

```rust
fn main() {
    let mut name = Some("JiangBo");
    name.replace("Rust");
    println!("{:?}", name);
}
```

### zip

```rust
fn main() {
    let name = Some("JiangBo");
    println!("{:?}", name.zip(Some("Rust")));
}
```

### copied

```rust
fn main() {
    let name = Some(&44);
    println!("{:?}", name.copied());
}
```

### clone

```rust
fn main() {
    let name = Some(&44);
    println!("{:?}", name.clone());
}
```

### transpose

```rust
fn main() {
    let name: Option<Result<&str, u32>> = Some(Ok("JiangBo"));
    println!("{:?}", name.transpose());
}
```

### flatten

```rust
fn main() {
    let name = Some(Some("JiangBo"));
    println!("{:?}", name.flatten());
}
```

## 总结

了解了可选枚举中相关的一些方法。

## 附录
