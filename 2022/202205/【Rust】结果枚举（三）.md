# 【Rust】结果枚举（三）

## 环境

- Time 2022-03-25
- Rust 1.59.0

## 示例

### or

```rust
fn main() {
    let result: Result<u32, u32> = Ok(44);
    println!("{:?}", result.or(Err(4)));
}
```

### or_else

```rust
fn main() {
    let result: Result<u32, u32> = Ok(44);
    let result: Result<u32, u32> = result.or_else(|e| Ok(e * 2));
    println!("{:?}", result);
}
```

### unwrap_or

```rust
fn main() {
    let result: Result<u32, u32> = Err(44);
    println!("{:?}", result.unwrap_or(4));
}
```

### unwrap_or_else

```rust
fn main() {
    let result: Result<u32, u32> = Err(44);
    println!("{:?}", result.unwrap_or_else(|x| x * 2));
}
```

### unwrap_unchecked

```rust
fn main() {
    let result: Result<u32, u32> = Ok(44);
    println!("{:?}", unsafe { result.unwrap_unchecked() });
}
```

### unwrap_err_unchecked

```rust
fn main() {
    let result: Result<u32, u32> = Err(44);
    println!("{:?}", unsafe { result.unwrap_err_unchecked() });
}
```

## 总结

了解了结果枚举中相关的一些方法。

## 附录
