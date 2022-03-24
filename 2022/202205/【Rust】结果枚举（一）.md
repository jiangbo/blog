# 【Rust】结果枚举（一）

## 环境

- Time 2022-03-24
- Rust 1.59.0

## 示例

### is_ok

```rust
fn main() {
    let result: Result<i32, &str> = Ok(44);
    println!("{:?}", result.is_ok());
}
```

### is_err

```rust
fn main() {
    let result: Result<i32, &str> = Ok(44);
    println!("{:?}", result.is_err());
}
```

### ok

```rust
fn main() {
    let result: Result<i32, &str> = Ok(44);
    println!("{:?}", result.ok());
}
```

### err

```rust
fn main() {
    let result: Result<i32, &str> = Ok(44);
    println!("{:?}", result.err());
}
```

### as_ref

```rust
fn main() {
    let result: Result<i32, &str> = Ok(44);
    println!("{:?}", result.as_ref());
}
```

### map

```rust
fn main() {
    let result: Result<i32, &str> = Ok(44);
    println!("{:?}", result.map(|e| e / 4));
}
```

### map_or

```rust
fn main() {
    let result: Result<i32, &str> = Ok(44);
    println!("{:?}", result.map_or(22, |e| e / 4));
}
```

### map_or_else

```rust
fn main() {
    let result: Result<i32, &str> = Ok(44);
    println!("{:?}", result.map_or_else(|_| 2, |e| e / 4));
}
```

### map_err

```rust
fn main() {
    let result: Result<i32, &str> = Err("error");
    println!("{:?}", result.map_err(|e| format!("[{}]", e)));
}
```

## 总结

了解了哈希映射中相关的一些方法。

## 附录
