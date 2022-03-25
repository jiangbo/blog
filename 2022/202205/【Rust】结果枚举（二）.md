# 【Rust】结果枚举（二）

## 环境

- Time 2022-03-25
- Rust 1.59.0

## 示例

### as_deref

```rust
fn main() {
    let result: Result<String, u32> = Ok("ok".to_owned());
    let result: Result<&str, &u32> = result.as_deref();
    println!("{:?}", result);
}
```

### iter

```rust
fn main() {
    let result: Result<&str, u32> = Ok("ok");
    println!("{:?}", result.iter().next());
}
```

### expect

```rust
fn main() {
    let result: Result<&str, u32> = Ok("ok");
    println!("{:?}", result.expect("错误"));
}
```

### unwrap

```rust
fn main() {
    let result: Result<&str, u32> = Ok("ok");
    println!("{:?}", result.unwrap());
}
```

### unwrap_or_default

```rust
fn main() {
    let result: Result<&str, u32> = Err(44);
    println!("{:?}", result.unwrap_or_default());
}
```

### expect_err

```rust
fn main() {
    let result: Result<&str, u32> = Err(44);
    println!("{:?}", result.expect_err("OK"));
}
```

### unwrap_err

```rust
fn main() {
    let result: Result<&str, u32> = Err(44);
    println!("{:?}", result.unwrap_err());
}
```

### and

```rust
fn main() {
    let result: Result<u32, u32> = Ok(44);
    println!("{:?}", result.and(Ok(4)));
}
```

### and_then

```rust
fn main() {
    let result: Result<u32, u32> = Ok(44);
    let result: Result<u32, u32> = result.and_then(|e| Err(e * 2));
    println!("{:?}", result);
}
```

## 总结

了解了结果枚举中相关的一些方法。

## 附录
